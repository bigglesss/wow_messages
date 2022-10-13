use crate::file_info::FileInfo;
use crate::Tags;
use std::fmt::Write;
use std::process::exit;

const COMPLEX_NOT_FOUND: i32 = 1;

pub struct ErrorWriter {
    inner: String,
}

impl ErrorWriter {
    pub fn new(msg: &str) -> Self {
        let mut s = Self {
            inner: String::with_capacity(8000),
        };

        s.wln(format!("WOWM ERROR: {}", msg));

        s
    }

    pub fn w(&mut self, s: impl AsRef<str>) {
        self.inner.write_str(s.as_ref()).unwrap();
    }

    pub fn newline(&mut self) {
        self.inner.write_char('\n').unwrap();
    }

    pub fn wln(&mut self, s: impl AsRef<str>) {
        self.w(s);
        self.newline();
    }

    pub fn print(&self) {
        eprintln!("{}", self.inner)
    }
}

fn wowm_exit(s: ErrorWriter, code: i32) -> ! {
    #[cfg(not(test))]
    const TEST: bool = false;
    #[cfg(test)]
    const TEST: bool = true;

    if TEST {
        panic!("{}", code);
    } else {
        s.print();

        exit(code);
    }
}

pub(crate) fn complex_not_found(
    struct_name: &str,
    struct_tags: &Tags,
    struct_fileinfo: &FileInfo,
    ty_name: &str,
) -> ! {
    let mut s = ErrorWriter::new("Container has complex type that can not be found.");

    s.wln(format!(
        "{}:{}:",
        struct_fileinfo.name(),
        struct_fileinfo.start_line()
    ));
    s.wln(format!(
        "    Type '{}' needs type '{}'",
        struct_name, ty_name
    ));
    s.newline();
    s.wln(format!("    '{}' needs to cover versions:", ty_name));
    if !struct_tags.logon_versions().collect::<Vec<_>>().is_empty() {
        s.wln("    Login:");

        for t in struct_tags.logon_versions() {
            s.wln(format!("        {}", t));
        }
    }

    if !struct_tags.versions().collect::<Vec<_>>().is_empty() {
        s.wln("    World:");

        for t in struct_tags.versions() {
            s.wln(format!("        {}", t));
        }
    }

    wowm_exit(s, COMPLEX_NOT_FOUND);
}

pub(crate) fn variable_in_if_not_found(variable_name: &str, name: &str, ty_name: &str) -> ! {
    let s = ErrorWriter::new("Container uses enumerator in if statement that does not exist.");

    s.print();

    panic!(
        "unable to find enumerator with name '{}' in variable '{}' with type '{}'",
        name, variable_name, ty_name
    )
}
