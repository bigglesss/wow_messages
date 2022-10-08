use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Write};

use crate::file_utils::get_import_path;
use crate::rust_printer::Version;
use crate::{
    Objects, COMMENT, DESCRIPTION, COMPRESSED, DISPLAY, LOGIN_VERSIONS, RUST_BASE_TYPE, TEST_STR, VERSIONS,
};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub(crate) enum WorldVersion {
    Major(u8),
    Minor(u8, u8),
    Patch(u8, u8, u8),
    Exact(u8, u8, u8, u16),
    All,
}

impl PartialOrd<WorldVersion> for WorldVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for WorldVersion {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            WorldVersion::Major(m) => match other {
                WorldVersion::Major(om) => m.cmp(om),
                WorldVersion::Minor(om, _)
                | WorldVersion::Patch(om, _, _)
                | WorldVersion::Exact(om, _, _, _) => m.cmp(om).then(Ordering::Less),
                WorldVersion::All => Ordering::Greater,
            },
            WorldVersion::Minor(m, i) => match other {
                WorldVersion::Major(om) => m.cmp(om).then(Ordering::Greater),
                WorldVersion::Minor(om, oi) => m.cmp(om).then_with(|| i.cmp(oi)),
                WorldVersion::Patch(om, oi, _) | WorldVersion::Exact(om, oi, _, _) => {
                    m.cmp(om).then_with(|| i.cmp(oi)).then(Ordering::Less)
                }
                WorldVersion::All => Ordering::Greater,
            },
            WorldVersion::Patch(m, i, p) => match other {
                WorldVersion::Major(om) => m.cmp(om).then(Ordering::Greater),
                WorldVersion::Minor(om, oi) => {
                    m.cmp(om).then_with(|| i.cmp(oi)).then(Ordering::Greater)
                }
                WorldVersion::Patch(om, oi, op) => {
                    m.cmp(om).then_with(|| i.cmp(oi)).then_with(|| p.cmp(op))
                }
                WorldVersion::Exact(om, oi, op, _) => m
                    .cmp(om)
                    .then_with(|| i.cmp(oi))
                    .then_with(|| p.cmp(op))
                    .then(Ordering::Less),
                WorldVersion::All => Ordering::Greater,
            },
            WorldVersion::Exact(m, i, p, e) => match other {
                WorldVersion::Major(om) => m.cmp(om).then(Ordering::Greater),
                WorldVersion::Minor(om, oi) => {
                    m.cmp(om).then_with(|| i.cmp(oi)).then(Ordering::Greater)
                }
                WorldVersion::Patch(om, oi, op) => m
                    .cmp(om)
                    .then_with(|| i.cmp(oi))
                    .then_with(|| p.cmp(op))
                    .then(Ordering::Greater),
                WorldVersion::Exact(om, oi, op, oe) => m
                    .cmp(om)
                    .then_with(|| i.cmp(oi))
                    .then_with(|| p.cmp(op))
                    .then_with(|| e.cmp(oe)),
                WorldVersion::All => Ordering::Greater,
            },
            WorldVersion::All => match other {
                WorldVersion::All => Ordering::Equal,
                WorldVersion::Major(_)
                | WorldVersion::Minor(_, _)
                | WorldVersion::Patch(_, _, _)
                | WorldVersion::Exact(_, _, _, _) => Ordering::Less,
            },
        }
    }
}

#[test]
fn world_version_ordering() {
    let mut v = [
        WorldVersion::Exact(1, 12, 1, 0),
        WorldVersion::Patch(3, 3, 5),
        WorldVersion::Major(1),
        WorldVersion::Minor(2, 4),
        WorldVersion::Major(3),
    ];

    v.sort();

    let expected = [
        WorldVersion::Major(1),
        WorldVersion::Exact(1, 12, 1, 0),
        WorldVersion::Minor(2, 4),
        WorldVersion::Major(3),
        WorldVersion::Patch(3, 3, 5),
    ];

    assert_eq!(v, expected);
}

impl WorldVersion {
    /// self has some overlap with other
    pub(crate) fn overlaps(&self, other: &Self) -> bool {
        match self {
            WorldVersion::Major(m) => match other {
                WorldVersion::Major(om)
                | WorldVersion::Minor(om, _)
                | WorldVersion::Patch(om, _, _)
                | WorldVersion::Exact(om, _, _, _) => m == om,
                WorldVersion::All => true,
            },
            WorldVersion::Minor(m, i) => match other {
                WorldVersion::Major(om) => om == m,
                WorldVersion::Minor(om, oi) => om == m && oi == i,
                WorldVersion::Patch(om, oi, _) => om == m && oi == i,
                WorldVersion::Exact(om, oi, _, _) => om == m && oi == i,
                WorldVersion::All => true,
            },
            WorldVersion::Patch(m, i, p) => match other {
                WorldVersion::Major(om) => om == m,
                WorldVersion::Minor(om, oi) => om == m && oi == i,
                WorldVersion::Patch(om, oi, op) => om == m && oi == i && op == p,
                WorldVersion::Exact(om, oi, op, _) => om == m && oi == i && op == p,
                WorldVersion::All => true,
            },
            WorldVersion::Exact(m, i, p, e) => match other {
                WorldVersion::Major(om) => om == m,
                WorldVersion::Minor(om, oi) => om == m && oi == i,
                WorldVersion::Patch(om, oi, op) => om == m && oi == i && op == p,
                WorldVersion::Exact(om, oi, op, oe) => om == m && oi == i && op == p && oe == e,
                WorldVersion::All => true,
            },
            WorldVersion::All => true,
        }
    }

    /// self completely fulfills other
    pub(crate) fn covers(&self, other: &Self) -> bool {
        match other {
            WorldVersion::Major(om) => match self {
                WorldVersion::Major(m) => m == om,
                WorldVersion::Minor(_, _) => false,
                WorldVersion::Patch(_, _, _) => false,
                WorldVersion::Exact(_, _, _, _) => false,
                WorldVersion::All => true,
            },
            WorldVersion::Minor(om, oi) => match self {
                WorldVersion::Major(m) => m == om,
                WorldVersion::Minor(m, i) => m == om && i == oi,
                WorldVersion::Patch(_, _, _) => false,
                WorldVersion::Exact(_, _, _, _) => false,
                WorldVersion::All => true,
            },
            WorldVersion::Patch(om, oi, op) => match self {
                WorldVersion::Major(m) => m == om,
                WorldVersion::Minor(m, i) => m == om && i == oi,
                WorldVersion::Patch(m, i, p) => m == om && i == oi && p == op,
                WorldVersion::Exact(_, _, _, _) => false,
                WorldVersion::All => true,
            },
            WorldVersion::Exact(om, oi, op, oe) => match self {
                WorldVersion::Major(m) => m == om,
                WorldVersion::Minor(m, i) => m == om && i == oi,
                WorldVersion::Patch(m, i, p) => m == om && i == oi && p == op,
                WorldVersion::Exact(m, i, p, e) => m == om && i == oi && p == op && e == oe,
                WorldVersion::All => true,
            },
            WorldVersion::All => matches!(self, WorldVersion::All),
        }
    }

    pub(crate) fn is_main_version(&self) -> bool {
        let mains = [
            WorldVersion::Minor(1, 12),
            WorldVersion::Patch(2, 4, 3),
            WorldVersion::Patch(3, 3, 5),
        ];

        for v in mains {
            if self.overlaps(&v) {
                return true;
            }
        }

        false
    }

    pub(crate) fn as_module_case(&self) -> String {
        self.to_string().replace('.', "_")
    }
}

impl Display for WorldVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WorldVersion::Major(m) => f.write_fmt(format_args!("{}", m)),
            WorldVersion::Minor(m, i) => f.write_fmt(format_args!("{}.{}", m, i)),
            WorldVersion::Patch(m, i, p) => f.write_fmt(format_args!("{}.{}.{}", m, i, p)),
            WorldVersion::Exact(m, i, p, b) => f.write_fmt(format_args!("{}.{}.{}.{}", m, i, p, b)),
            WorldVersion::All => f.write_str("*"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum LoginVersion {
    Specific(u8),
    All,
}

impl Display for LoginVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LoginVersion::Specific(v) => f.write_fmt(format_args!("{}", v)),
            LoginVersion::All => f.write_str("*"),
        }
    }
}

impl LoginVersion {
    pub(crate) fn as_module_case(&self) -> String {
        match self {
            LoginVersion::Specific(_) => self.to_string(),
            LoginVersion::All => "all".to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub(crate) struct Tags {
    inner: Vec<Tag>,
    login_logon_versions: Vec<LoginVersion>,
    world_versions: Vec<WorldVersion>,
    description: Option<TagString>,
    compressed: bool,
    comment: Option<TagString>,
    display: String,
}

impl Tags {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn new_with_tag(t: Tag) -> Self {
        let mut s = Self::new();
        s.push(t);
        s
    }

    pub(crate) fn new_with_version(version: Version) -> Self {
        let mut s = Self::new();
        match version {
            Version::Login(l) => s.login_logon_versions.push(l),
            Version::World(l) => s.world_versions.push(l),
        }
        s
    }

    pub(crate) fn push(&mut self, t: Tag) {
        self.append_or_insert(t.key(), t.value());
    }

    pub(crate) fn append(&mut self, t: &Tags) {
        for tag in &t.inner {
            self.append_or_insert(tag.key(), tag.value());
        }
    }

    pub(crate) fn append_or_insert(&mut self, key: &str, value: &str) {
        if key == LOGIN_VERSIONS {
            for w in value.split_whitespace() {
                if let Ok(v) = w.parse::<u8>() {
                    self.login_logon_versions.push(LoginVersion::Specific(v));
                } else if w == "*" {
                    self.login_logon_versions.push(LoginVersion::All);
                } else {
                    panic!("invalid value passed as login_logon_versions: '{}'", w);
                }
            }
            self.login_logon_versions.sort();
            self.login_logon_versions.dedup();
            if self.login_logon_versions.contains(&LoginVersion::All) {
                self.login_logon_versions = vec![LoginVersion::All];
            }

            //return;
        } else if key == VERSIONS {
            for w in value.split_whitespace() {
                if let Ok(v) = w.parse::<u8>() {
                    self.world_versions.push(WorldVersion::Major(v));
                    continue;
                } else if w == "*" {
                    self.world_versions.push(WorldVersion::All);
                    continue;
                }

                let d: Vec<u8> = w.split('.').map(|a| a.parse::<u8>().unwrap()).collect();
                self.world_versions.push(match d.len() {
                    2 => WorldVersion::Minor(d[0], d[1]),
                    3 => WorldVersion::Patch(d[0], d[1], d[2]),
                    4 => WorldVersion::Exact(d[0], d[1], d[2], u16::from(d[3])),
                    _ => panic!("incorrect world version string"),
                });
            }
            self.world_versions.sort();
            self.world_versions.dedup();
            if self.world_versions.contains(&WorldVersion::All) {
                self.world_versions = vec![WorldVersion::All];
            }
        } else if key == DESCRIPTION {
            if let Some(desc) = &mut self.description {
                desc.add(value);
            } else {
                let mut t = TagString::new();
                t.add(value);
                self.description = Some(t);
            }
        } else if key == COMPRESSED {
            self.compressed = value == "true";
        } else if key == COMMENT {
            if let Some(comment) = &mut self.comment {
                comment.add(value);
            } else {
                let mut t = TagString::new();
                t.add(value);
                self.comment = Some(t);
            }
        } else if key == DISPLAY {
            self.display = value.to_string();
        }

        for v in self.inner.iter_mut() {
            if v.key == key {
                v.value += " ";
                v.value += value;
                return;
            }
        }

        self.inner.push(Tag::new(key, value));
    }

    pub(crate) fn contains(&self, name: &str) -> bool {
        self.inner.iter().any(|a| a.key == name)
    }

    pub(crate) fn get_ref(&self, name: &str) -> Option<&str> {
        match self.inner.iter().find(|a| a.key == name) {
            None => None,
            Some(v) => Some(v.value()),
        }
    }

    pub(crate) fn unimplemented(&self) -> bool {
        self.contains("unimplemented")
    }

    pub(crate) fn shared(&self) -> bool {
        self.main_versions().count() != 1
    }

    /// self and tags have any version in common at all
    pub(crate) fn has_version_intersections(&self, tags: &Tags) -> bool {
        if tags.contains(TEST_STR) && self.contains(TEST_STR) {
            return true;
        }

        if ((tags.has_login_version() && self.has_wildcard_logon_version())
            || (self.has_login_version() && tags.has_wildcard_logon_version()))
            || ((tags.has_world_version() && self.has_wildcard_world_version())
                || (self.has_world_version() && tags.has_wildcard_world_version()))
        {
            return true;
        }

        for t in self.logon_versions() {
            if tags.logon_versions().contains(t) {
                return true;
            }
        }

        for outer in self.versions() {
            for inner in tags.versions() {
                if outer.overlaps(inner) {
                    return true;
                }
            }
        }

        false
    }

    pub(crate) fn is_main_version(&self) -> bool {
        let versions = Tags::new_with_tag(Tag::new(VERSIONS, "1.12 2.4.3 3.3.5"));
        let logon = Tags::new_with_tag(Tag::new(LOGIN_VERSIONS, "*"));

        self.has_version_intersections(&versions) || self.has_version_intersections(&logon)
    }

    pub(crate) fn has_wildcard_logon_version(&self) -> bool {
        self.logon_versions().contains(&LoginVersion::All)
    }

    pub(crate) fn has_wildcard_world_version(&self) -> bool {
        self.versions().contains(&WorldVersion::All)
    }

    pub(crate) fn logon_versions(&self) -> &[LoginVersion] {
        &self.login_logon_versions
    }

    pub(crate) fn versions(&self) -> &[WorldVersion] {
        &self.world_versions
    }

    pub(crate) fn main_versions(&self) -> impl Iterator<Item = Version> + '_ {
        let world = self
            .versions()
            .iter()
            .filter(|a| a.is_main_version())
            .map(|a| Version::World(*a));

        self.logon_versions()
            .iter()
            .map(|a| Version::Login(*a))
            .chain(world)
    }

    pub(crate) fn main_trait_versions(&self) -> Vec<Version> {
        let mut v = Vec::new();

        if self.fulfills_all(&Tags::new_with_version(Version::World(
            WorldVersion::Minor(1, 12),
        ))) {
            v.push(Version::World(WorldVersion::Minor(1, 12)));
        }
        if self.fulfills_all(&Tags::new_with_version(Version::World(
            WorldVersion::Patch(2, 4, 3),
        ))) {
            v.push(Version::World(WorldVersion::Patch(2, 4, 3)))
        }

        if self.fulfills_all(&Tags::new_with_version(Version::World(
            WorldVersion::Patch(3, 3, 5),
        ))) {
            v.push(Version::World(WorldVersion::Patch(3, 3, 5)));
        }

        v
    }

    pub(crate) fn first_version(&self) -> Version {
        if let Some(v) = self.logon_versions().first() {
            Version::Login(*v)
        } else if let Some(v) = self.versions().first() {
            Version::World(*v)
        } else {
            unreachable!()
        }
    }

    pub(crate) fn first_and_main_versions(&self) -> (Version, Vec<Version>) {
        let mut v = self.main_versions();
        let first = v.next().unwrap();

        let rest = v.collect::<Vec<_>>();

        (first, rest)
    }

    pub(crate) fn import_version(&self) -> Version {
        let (first, _) = self.first_and_main_versions();

        first
    }

    pub(crate) fn has_world_version(&self) -> bool {
        if !self.versions().is_empty() {
            assert!(self.logon_versions().is_empty());
            return true;
        } else if !self.logon_versions().is_empty() {
            assert!(self.versions().is_empty());
            return false;
        }

        false
    }

    /// self is able to fulfill all version obligations for tags
    pub(crate) fn fulfills_all(&self, tags: &Self) -> bool {
        for version in tags.logon_versions() {
            if !self.logon_versions().contains(version)
                && !self.logon_versions().contains(&LoginVersion::All)
            {
                return false;
            }
        }

        for version in tags.versions() {
            let mut covered = false;

            for self_version in self.versions() {
                if self_version.covers(version) {
                    covered = true;
                }
            }

            if !covered {
                return false;
            }
        }

        true
    }

    pub(crate) fn has_login_version(&self) -> bool {
        if !self.login_logon_versions.is_empty() {
            assert!(self.world_versions.is_empty());
            return true;
        } else if !self.world_versions.is_empty() {
            assert!(self.login_logon_versions.is_empty());
            return false;
        }

        false
    }

    pub(crate) fn description(&self) -> Option<&TagString> {
        self.description.as_ref()
    }

    pub(crate) fn is_compressed(&self) -> bool {
        self.compressed
    }

    pub(crate) fn comment(&self) -> Option<&TagString> {
        self.comment.as_ref()
    }

    pub(crate) fn display(&self) -> Option<&str> {
        if self.display.is_empty() {
            None
        } else {
            Some(&self.display)
        }
    }

    pub(crate) fn is_in_base(&self) -> bool {
        self.contains(RUST_BASE_TYPE)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct Tag {
    key: String,
    value: String,
}

impl Tag {
    pub(crate) fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.to_owned(),
            value: value.to_owned(),
        }
    }

    pub(crate) fn key(&self) -> &str {
        &self.key
    }

    pub(crate) fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum TagStringSymbol {
    Text(String),
    Link(String),
    Newline,
}

impl TagStringSymbol {
    fn from_str(s: &str) -> Vec<Self> {
        if !s.contains('[') {
            return vec![TagStringSymbol::Text(s.to_string())];
        }

        let mut v = Vec::new();
        let mut s = s.to_string();

        while let Some(start) = s.find('[') {
            let end = s.find(']').unwrap();
            v.push(TagStringSymbol::Text(s[..start].to_string()));
            v.push(TagStringSymbol::Link(s[start + 1..end].to_string()));

            s = s[end + 1..].to_string();
        }
        if !s.is_empty() {
            v.push(TagStringSymbol::Text(s));
        }

        v
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub(crate) struct TagString {
    inner: Vec<TagStringSymbol>,
}

impl TagString {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn add(&mut self, s: &str) {
        if !self.inner.is_empty() {
            self.inner.push(TagStringSymbol::Newline);
        }

        self.inner.append(&mut TagStringSymbol::from_str(s));
    }

    pub(crate) fn as_doc_lines(&self) -> Vec<String> {
        let mut v = Vec::new();

        let mut current = String::new();
        for i in &self.inner {
            match i {
                TagStringSymbol::Text(s) => current.push_str(s),
                TagStringSymbol::Link(s) => write!(
                    current,
                    "[{name}](./{lower}.md)",
                    name = s,
                    lower = s.to_lowercase()
                )
                .unwrap(),
                TagStringSymbol::Newline => {
                    v.push(current.clone());
                    current.clear();
                }
            }
        }

        v.push(current);

        v
    }

    pub(crate) fn as_rust_doc_lines(&self, o: &Objects, object_tags: &Tags) -> Vec<String> {
        let mut v = Vec::new();

        let mut current = String::new();

        for i in &self.inner {
            match i {
                TagStringSymbol::Text(s) => current.push_str(s),
                TagStringSymbol::Link(s) => {
                    if let Some(tags) = o.get_tags_of_object_fallible(s, object_tags) {
                        let version = tags.import_version();
                        write!(current, "[`{}`]({}::{})", s, get_import_path(version), s).unwrap()
                    } else {
                        write!(current, "`{}`", s).unwrap()
                    }
                }
                TagStringSymbol::Newline => {
                    v.push(current.clone());
                    current.clear();
                }
            }
        }

        v.push(current);

        v
    }

    pub(crate) fn as_doc_table_string(&self) -> String {
        let mut s = String::new();

        for i in &self.inner {
            match i {
                TagStringSymbol::Text(t) => s.push_str(t),
                TagStringSymbol::Link(l) => {
                    write!(s, "[{}](./{}.md)", l, l.to_lowercase()).unwrap()
                }
                TagStringSymbol::Newline => s.push_str("<br/>"),
            }
        }

        s
    }

    pub(crate) fn as_ir_string(&self) -> String {
        let mut s = String::new();

        for i in &self.inner {
            match i {
                TagStringSymbol::Text(t) => s.push_str(t),
                TagStringSymbol::Link(l) => write!(s, "[{}]", l).unwrap(),
                TagStringSymbol::Newline => s.push('\n'),
            }
        }

        s
    }
}
