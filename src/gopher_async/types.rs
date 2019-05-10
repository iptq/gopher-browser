/// The type of a resource in a Gopher directory.
///
/// For more details, see section 3.8 of https://tools.ietf.org/html/rfc1436
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
#[allow(dead_code)]
pub enum ItemType {
    /// Item is a file
    File,
    /// Item is a directory
    Dir,
    /// Item is a CSO phone-book server
    CsoServer,
    /// Error
    Error,
    /// Item is a BinHexed Macintosh file.
    BinHex,
    /// Item is DOS binary archive of some sort.
    ///
    /// Client must read until the TCP connection closes.  Beware.
    Dos,
    /// Item is a UNIX uuencoded file.
    Uuencoded,
    /// Item is an Index-Search server.
    IndexServer,
    /// Item points to a text-based telnet session.
    Telnet,
    /// Item is a binary file! Client must read until the TCP connection closes.  Beware
    Binary,
    /// Item is a redundant server
    RedundantServer,
    /// Item points to a text-based tn3270 session.
    Tn3270,
    /// Item is a GIF format graphics file.
    Gif,
    /// Item is some kind of image file.  Client decides how to display.
    Image,
    /// Item is a non-standard type
    Other(u8),
}

impl ItemType {
    #[allow(dead_code)]
    pub fn decode(b: u8) -> Self {
        use ItemType::*;
        match b {
            b'0' => File,
            b'1' => Dir,
            b'2' => CsoServer,
            b'3' => Error,
            b'4' => BinHex,
            b'5' => Dos,
            b'6' => Uuencoded,
            b'7' => IndexServer,
            b'8' => Telnet,
            b'9' => Binary,
            b'+' => RedundantServer,
            b'T' => Tn3270,
            b'g' => Gif,
            b'I' => Image,
            byte => Other(byte),
        }
    }

    #[allow(dead_code)]
    pub fn encode(self) -> u8 {
        use ItemType::*;
        match self {
            File => b'0',
            Dir => b'1',
            CsoServer => b'2',
            Error => b'3',
            BinHex => b'4',
            Dos => b'5',
            Uuencoded => b'6',
            IndexServer => b'7',
            Telnet => b'8',
            Binary => b'9',
            RedundantServer => b'+',
            Tn3270 => b'T',
            Gif => b'g',
            Image => b'I',
            Other(byte) => byte,
        }
    }
}
