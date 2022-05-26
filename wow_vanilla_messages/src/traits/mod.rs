use std::future::Future;
use std::pin::Pin;
use wow_srp::header_crypto::Encrypter;

const SERVER_OPCODE_LENGTH: u16 = 2;
const SERVER_HEADER_LENGTH: u16 = 4;
const CLIENT_OPCODE_LENGTH: u16 = 4;
const CLIENT_HEADER_LENGTH: u16 = 6;

fn get_unencrypted_server(opcode: u16, size: u16, mut bytes: Vec<u8>) -> Vec<u8> {
    let mut v = Vec::with_capacity((size + SERVER_HEADER_LENGTH) as usize);

    crate::util::write_u16_be(&mut v, size + SERVER_OPCODE_LENGTH).unwrap();
    crate::util::write_u16_le(&mut v, opcode).unwrap();

    v.append(&mut bytes);

    v
}

fn get_encrypted_server(
    opcode: u16,
    size: u16,
    mut bytes: Vec<u8>,
    e: &mut impl Encrypter,
) -> Vec<u8> {
    let mut v = Vec::with_capacity((size + SERVER_HEADER_LENGTH) as usize);

    v.extend_from_slice(&e.encrypt_server_header(size + SERVER_OPCODE_LENGTH, opcode));

    v.append(&mut bytes);
    v
}

pub trait ServerMessage: Sized {
    const OPCODE: u16;

    fn size_without_size_or_opcode_fields(&self) -> u16;

    type Error;

    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error>;

    #[cfg(feature = "sync")]
    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        let v = get_unencrypted_server(
            Self::OPCODE,
            self.size_without_size_or_opcode_fields(),
            self.as_bytes()?,
        );

        w.write_all(&v)
    }

    #[cfg(feature = "sync")]
    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> Result<(), std::io::Error> {
        let v = get_encrypted_server(
            Self::OPCODE,
            self.size_without_size_or_opcode_fields(),
            self.as_bytes()?,
            e,
        );

        w.write_all(&v)
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_unencrypted_server<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let v = get_unencrypted_server(
                Self::OPCODE,
                self.size_without_size_or_opcode_fields(),
                self.as_bytes()?,
            );

            w.write_all(&v).await
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_encrypted_server<'life0, 'life1, 'life2, 'async_trait, W, E>(
        &'life0 self,
        w: &'life1 mut W,
        e: &'life2 mut E,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        E: 'async_trait + Encrypter + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let v = get_encrypted_server(
                Self::OPCODE,
                self.size_without_size_or_opcode_fields(),
                self.as_bytes()?,
                e,
            );

            w.write_all(&v).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_unencrypted_server<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let v = get_unencrypted_server(
                Self::OPCODE,
                self.size_without_size_or_opcode_fields(),
                self.as_bytes()?,
            );

            w.write_all(&v).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_encrypted_server<'life0, 'life1, 'life2, 'async_trait, W, E>(
        &'life0 self,
        w: &'life1 mut W,
        e: &'life2 mut E,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        E: 'async_trait + Encrypter + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let v = get_encrypted_server(
                Self::OPCODE,
                self.size_without_size_or_opcode_fields(),
                self.as_bytes()?,
                e,
            );

            w.write_all(&v).await
        })
    }

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> Result<Self, Self::Error>;

    #[cfg(feature = "async-std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::Error>> + Send + 'async_trait>>
    where
        R: 'async_trait + async_std::io::ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait;

    #[cfg(feature = "tokio")]
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::Error>> + Send + 'async_trait>>
    where
        R: 'async_trait + tokio::io::AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait;
}

pub trait ClientMessage: Sized {
    const OPCODE: u16;

    fn size_without_size_or_opcode_fields(&self) -> u16;

    type Error;

    fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error>;

    #[cfg(feature = "sync")]
    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        let v = get_unencrypted_client(
            Self::OPCODE,
            self.size_without_size_or_opcode_fields(),
            self.as_bytes()?,
        );

        w.write_all(&v)
    }

    #[cfg(feature = "sync")]
    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> Result<(), std::io::Error> {
        let v = get_encrypted_client(
            Self::OPCODE,
            self.size_without_size_or_opcode_fields(),
            self.as_bytes()?,
            e,
        );

        w.write_all(&v)
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_unencrypted_client<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let v = get_unencrypted_client(
                Self::OPCODE,
                self.size_without_size_or_opcode_fields(),
                self.as_bytes()?,
            );

            w.write_all(&v).await
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_encrypted_client<'life0, 'life1, 'life2, 'async_trait, W, E>(
        &'life0 self,
        w: &'life1 mut W,
        e: &'life2 mut E,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        E: 'async_trait + Encrypter + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let v = get_encrypted_client(
                Self::OPCODE,
                self.size_without_size_or_opcode_fields(),
                self.as_bytes()?,
                e,
            );

            w.write_all(&v).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_unencrypted_client<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let v = get_unencrypted_client(
                Self::OPCODE,
                self.size_without_size_or_opcode_fields(),
                self.as_bytes()?,
            );

            w.write_all(&v).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_encrypted_client<'life0, 'life1, 'life2, 'async_trait, W, E>(
        &'life0 self,
        w: &'life1 mut W,
        e: &'life2 mut E,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        E: 'async_trait + Encrypter + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let v = get_encrypted_client(
                Self::OPCODE,
                self.size_without_size_or_opcode_fields(),
                self.as_bytes()?,
                e,
            );

            w.write_all(&v).await
        })
    }

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> Result<Self, Self::Error>;

    #[cfg(feature = "async-std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::Error>> + Send + 'async_trait>>
    where
        R: 'async_trait + async_std::io::ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait;

    #[cfg(feature = "tokio")]
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::Error>> + Send + 'async_trait>>
    where
        R: 'async_trait + tokio::io::AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait;
}

fn get_unencrypted_client(opcode: u16, size: u16, mut bytes: Vec<u8>) -> Vec<u8> {
    let mut v = Vec::with_capacity((size + CLIENT_HEADER_LENGTH) as usize);

    crate::util::write_u16_be(&mut v, size + CLIENT_OPCODE_LENGTH).unwrap();
    crate::util::write_u32_le(&mut v, opcode as u32).unwrap();

    v.append(&mut bytes);

    v
}

fn get_encrypted_client(
    opcode: u16,
    size: u16,
    mut bytes: Vec<u8>,
    e: &mut impl Encrypter,
) -> Vec<u8> {
    let mut v = Vec::with_capacity((size + CLIENT_HEADER_LENGTH) as usize);

    v.extend_from_slice(&e.encrypt_client_header(size + CLIENT_OPCODE_LENGTH, opcode as u32));

    v.append(&mut bytes);
    v
}
