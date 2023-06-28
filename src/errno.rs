#![allow(dead_code)]

// This file automatically generate. Do not edit.

use crate::Errno;
impl Errno {
    /// kernel internal error
    pub const EDATALESS: Self = Self(-8);
    /// kernel internal error
    pub const EKEEPLOOKING: Self = Self(-7);
    /// kernel internal error
    pub const EREDRIVEOPEN: Self = Self(-6);
    /// restart lookup under heavy vnode pressure/recycling
    pub const ERECYCLE: Self = Self(-5);
    /// don't modify regs, just return
    pub const EJUSTRETURN: Self = Self(-2);
    /// restart syscall
    pub const ERESTART: Self = Self(-1);
    /// Operation not permitted
    pub const EPERM: Self = Self(1);
    /// No such file or directory
    pub const ENOENT: Self = Self(2);
    /// No such process
    pub const ESRCH: Self = Self(3);
    /// Interrupted system call
    pub const EINTR: Self = Self(4);
    /// Input/output error
    pub const EIO: Self = Self(5);
    /// Device not configured
    pub const ENXIO: Self = Self(6);
    /// Argument list too long
    pub const E2BIG: Self = Self(7);
    /// Exec format error
    pub const ENOEXEC: Self = Self(8);
    /// Bad file descriptor
    pub const EBADF: Self = Self(9);
    /// No child processes
    pub const ECHILD: Self = Self(10);
    /// Resource deadlock avoided
    pub const EDEADLK: Self = Self(11);
    /// Cannot allocate memory
    pub const ENOMEM: Self = Self(12);
    /// Permission denied
    pub const EACCES: Self = Self(13);
    /// Bad address
    pub const EFAULT: Self = Self(14);
    /// Block device required
    pub const ENOTBLK: Self = Self(15);
    /// Device / Resource busy
    pub const EBUSY: Self = Self(16);
    /// File exists
    pub const EEXIST: Self = Self(17);
    /// Cross-device link
    pub const EXDEV: Self = Self(18);
    /// Operation not supported by device
    pub const ENODEV: Self = Self(19);
    /// Not a directory
    pub const ENOTDIR: Self = Self(20);
    /// Is a directory
    pub const EISDIR: Self = Self(21);
    /// Invalid argument
    pub const EINVAL: Self = Self(22);
    /// Too many open files in system
    pub const ENFILE: Self = Self(23);
    /// Too many open files
    pub const EMFILE: Self = Self(24);
    /// Inappropriate ioctl for device
    pub const ENOTTY: Self = Self(25);
    /// Text file busy
    pub const ETXTBSY: Self = Self(26);
    /// File too large
    pub const EFBIG: Self = Self(27);
    /// No space left on device
    pub const ENOSPC: Self = Self(28);
    /// Illegal seek
    pub const ESPIPE: Self = Self(29);
    /// Read-only file system
    pub const EROFS: Self = Self(30);
    /// Too many links
    pub const EMLINK: Self = Self(31);
    /// Broken pipe
    pub const EPIPE: Self = Self(32);
    /// Numerical argument out of domain
    pub const EDOM: Self = Self(33);
    /// Result too large
    pub const ERANGE: Self = Self(34);
    /// Resource temporarily unavailable
    pub const EAGAIN: Self = Self(35);
    /// Operation now in progress
    pub const EINPROGRESS: Self = Self(36);
    /// Operation already in progress
    pub const EALREADY: Self = Self(37);
    /// Socket operation on non-socket
    pub const ENOTSOCK: Self = Self(38);
    /// Destination address required
    pub const EDESTADDRREQ: Self = Self(39);
    /// Message too long
    pub const EMSGSIZE: Self = Self(40);
    /// Protocol wrong type for socket
    pub const EPROTOTYPE: Self = Self(41);
    /// Protocol not available
    pub const ENOPROTOOPT: Self = Self(42);
    /// Protocol not supported
    pub const EPROTONOSUPPORT: Self = Self(43);
    /// Socket type not supported
    pub const ESOCKTNOSUPPORT: Self = Self(44);
    /// Operation not supported
    pub const ENOTSUP: Self = Self(45);
    /// Protocol family not supported
    pub const EPFNOSUPPORT: Self = Self(46);
    /// Address family not supported by protocol family
    pub const EAFNOSUPPORT: Self = Self(47);
    /// Address already in use
    pub const EADDRINUSE: Self = Self(48);
    /// Can't assign requested address
    pub const EADDRNOTAVAIL: Self = Self(49);
    /// Network is down
    pub const ENETDOWN: Self = Self(50);
    /// Network is unreachable
    pub const ENETUNREACH: Self = Self(51);
    /// Network dropped connection on reset
    pub const ENETRESET: Self = Self(52);
    /// Software caused connection abort
    pub const ECONNABORTED: Self = Self(53);
    /// Connection reset by peer
    pub const ECONNRESET: Self = Self(54);
    /// No buffer space available
    pub const ENOBUFS: Self = Self(55);
    /// Socket is already connected
    pub const EISCONN: Self = Self(56);
    /// Socket is not connected
    pub const ENOTCONN: Self = Self(57);
    /// Can't send after socket shutdown
    pub const ESHUTDOWN: Self = Self(58);
    /// Too many references: can't splice
    pub const ETOOMANYREFS: Self = Self(59);
    /// Operation timed out
    pub const ETIMEDOUT: Self = Self(60);
    /// Connection refused
    pub const ECONNREFUSED: Self = Self(61);
    /// Too many levels of symbolic links
    pub const ELOOP: Self = Self(62);
    /// File name too long
    pub const ENAMETOOLONG: Self = Self(63);
    /// Host is down
    pub const EHOSTDOWN: Self = Self(64);
    /// No route to host
    pub const EHOSTUNREACH: Self = Self(65);
    /// Directory not empty
    pub const ENOTEMPTY: Self = Self(66);
    /// Too many processes
    pub const EPROCLIM: Self = Self(67);
    /// Too many users
    pub const EUSERS: Self = Self(68);
    /// Disc quota exceeded
    pub const EDQUOT: Self = Self(69);
    /// Stale NFS file handle
    pub const ESTALE: Self = Self(70);
    /// Too many levels of remote in path
    pub const EREMOTE: Self = Self(71);
    /// RPC struct is bad
    pub const EBADRPC: Self = Self(72);
    /// RPC version wrong
    pub const ERPCMISMATCH: Self = Self(73);
    /// RPC prog. not avail
    pub const EPROGUNAVAIL: Self = Self(74);
    /// Program version wrong
    pub const EPROGMISMATCH: Self = Self(75);
    /// Bad procedure for program
    pub const EPROCUNAVAIL: Self = Self(76);
    /// No locks available
    pub const ENOLCK: Self = Self(77);
    /// Function not implemented
    pub const ENOSYS: Self = Self(78);
    /// Inappropriate file type or format
    pub const EFTYPE: Self = Self(79);
    /// Authentication error
    pub const EAUTH: Self = Self(80);
    /// Need authenticator
    pub const ENEEDAUTH: Self = Self(81);
    /// Device power is off
    pub const EPWROFF: Self = Self(82);
    /// Device error, e.g. paper out
    pub const EDEVERR: Self = Self(83);
    /// Value too large to be stored in data type
    pub const EOVERFLOW: Self = Self(84);
    /// Bad executable
    pub const EBADEXEC: Self = Self(85);
    /// Bad CPU type in executable
    pub const EBADARCH: Self = Self(86);
    /// Shared library version mismatch
    pub const ESHLIBVERS: Self = Self(87);
    /// Malformed Macho file
    pub const EBADMACHO: Self = Self(88);
    /// Operation canceled
    pub const ECANCELED: Self = Self(89);
    /// Identifier removed
    pub const EIDRM: Self = Self(90);
    /// No message of desired type
    pub const ENOMSG: Self = Self(91);
    /// Illegal byte sequence
    pub const EILSEQ: Self = Self(92);
    /// Attribute not found
    pub const ENOATTR: Self = Self(93);
    /// Bad message
    pub const EBADMSG: Self = Self(94);
    /// Reserved
    pub const EMULTIHOP: Self = Self(95);
    /// No message available on STREAM
    pub const ENODATA: Self = Self(96);
    /// Reserved
    pub const ENOLINK: Self = Self(97);
    /// No STREAM resources
    pub const ENOSR: Self = Self(98);
    /// Not a STREAM
    pub const ENOSTR: Self = Self(99);
    /// Protocol error
    pub const EPROTO: Self = Self(100);
    /// STREAM ioctl timeout
    pub const ETIME: Self = Self(101);
    /// Operation not supported on socket
    pub const EOPNOTSUPP: Self = Self(102);
    /// No such policy registered
    pub const ENOPOLICY: Self = Self(103);
    /// State not recoverable
    pub const ENOTRECOVERABLE: Self = Self(104);
    /// Previous owner died
    pub const EOWNERDEAD: Self = Self(105);
    /// Interface output queue is full
    pub const EQFULL: Self = Self(106);
    /// kernel internal error
    pub const ECVCERORR: Self = Self(256);
    /// kernel internal error
    pub const ECVPERORR: Self = Self(512);
    /// Alias for [Self::EAGAIN]
    pub const EWOULDBLOCK: Self = Self::EAGAIN;

    pub const MIN: i32 = -8;
    pub const MAX: i32 = 512;

    #[cfg(feature = "iter")]
    const ALL: [i32; 114] = [
        -8, -7, -6, -5, -2, -1, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42,
        43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65,
        66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88,
        89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 256, 512,
    ];

    pub(crate) fn name_and_description(&self) -> Option<(&'static str, &'static str)> {
        match self.0 {
            -8 => Some(("EDATALESS", "kernel internal error")),
            -7 => Some(("EKEEPLOOKING", "kernel internal error")),
            -6 => Some(("EREDRIVEOPEN", "kernel internal error")),
            -5 => Some((
                "ERECYCLE",
                "restart lookup under heavy vnode pressure/recycling",
            )),
            -2 => Some(("EJUSTRETURN", "don't modify regs, just return")),
            -1 => Some(("ERESTART", "restart syscall")),
            1 => Some(("EPERM", "Operation not permitted")),
            2 => Some(("ENOENT", "No such file or directory")),
            3 => Some(("ESRCH", "No such process")),
            4 => Some(("EINTR", "Interrupted system call")),
            5 => Some(("EIO", "Input/output error")),
            6 => Some(("ENXIO", "Device not configured")),
            7 => Some(("E2BIG", "Argument list too long")),
            8 => Some(("ENOEXEC", "Exec format error")),
            9 => Some(("EBADF", "Bad file descriptor")),
            10 => Some(("ECHILD", "No child processes")),
            11 => Some(("EDEADLK", "Resource deadlock avoided")),
            12 => Some(("ENOMEM", "Cannot allocate memory")),
            13 => Some(("EACCES", "Permission denied")),
            14 => Some(("EFAULT", "Bad address")),
            15 => Some(("ENOTBLK", "Block device required")),
            16 => Some(("EBUSY", "Device / Resource busy")),
            17 => Some(("EEXIST", "File exists")),
            18 => Some(("EXDEV", "Cross-device link")),
            19 => Some(("ENODEV", "Operation not supported by device")),
            20 => Some(("ENOTDIR", "Not a directory")),
            21 => Some(("EISDIR", "Is a directory")),
            22 => Some(("EINVAL", "Invalid argument")),
            23 => Some(("ENFILE", "Too many open files in system")),
            24 => Some(("EMFILE", "Too many open files")),
            25 => Some(("ENOTTY", "Inappropriate ioctl for device")),
            26 => Some(("ETXTBSY", "Text file busy")),
            27 => Some(("EFBIG", "File too large")),
            28 => Some(("ENOSPC", "No space left on device")),
            29 => Some(("ESPIPE", "Illegal seek")),
            30 => Some(("EROFS", "Read-only file system")),
            31 => Some(("EMLINK", "Too many links")),
            32 => Some(("EPIPE", "Broken pipe")),
            33 => Some(("EDOM", "Numerical argument out of domain")),
            34 => Some(("ERANGE", "Result too large")),
            35 => Some(("EAGAIN", "Resource temporarily unavailable")),
            36 => Some(("EINPROGRESS", "Operation now in progress")),
            37 => Some(("EALREADY", "Operation already in progress")),
            38 => Some(("ENOTSOCK", "Socket operation on non-socket")),
            39 => Some(("EDESTADDRREQ", "Destination address required")),
            40 => Some(("EMSGSIZE", "Message too long")),
            41 => Some(("EPROTOTYPE", "Protocol wrong type for socket")),
            42 => Some(("ENOPROTOOPT", "Protocol not available")),
            43 => Some(("EPROTONOSUPPORT", "Protocol not supported")),
            44 => Some(("ESOCKTNOSUPPORT", "Socket type not supported")),
            45 => Some(("ENOTSUP", "Operation not supported")),
            46 => Some(("EPFNOSUPPORT", "Protocol family not supported")),
            47 => Some((
                "EAFNOSUPPORT",
                "Address family not supported by protocol family",
            )),
            48 => Some(("EADDRINUSE", "Address already in use")),
            49 => Some(("EADDRNOTAVAIL", "Can't assign requested address")),
            50 => Some(("ENETDOWN", "Network is down")),
            51 => Some(("ENETUNREACH", "Network is unreachable")),
            52 => Some(("ENETRESET", "Network dropped connection on reset")),
            53 => Some(("ECONNABORTED", "Software caused connection abort")),
            54 => Some(("ECONNRESET", "Connection reset by peer")),
            55 => Some(("ENOBUFS", "No buffer space available")),
            56 => Some(("EISCONN", "Socket is already connected")),
            57 => Some(("ENOTCONN", "Socket is not connected")),
            58 => Some(("ESHUTDOWN", "Can't send after socket shutdown")),
            59 => Some(("ETOOMANYREFS", "Too many references: can't splice")),
            60 => Some(("ETIMEDOUT", "Operation timed out")),
            61 => Some(("ECONNREFUSED", "Connection refused")),
            62 => Some(("ELOOP", "Too many levels of symbolic links")),
            63 => Some(("ENAMETOOLONG", "File name too long")),
            64 => Some(("EHOSTDOWN", "Host is down")),
            65 => Some(("EHOSTUNREACH", "No route to host")),
            66 => Some(("ENOTEMPTY", "Directory not empty")),
            67 => Some(("EPROCLIM", "Too many processes")),
            68 => Some(("EUSERS", "Too many users")),
            69 => Some(("EDQUOT", "Disc quota exceeded")),
            70 => Some(("ESTALE", "Stale NFS file handle")),
            71 => Some(("EREMOTE", "Too many levels of remote in path")),
            72 => Some(("EBADRPC", "RPC struct is bad")),
            73 => Some(("ERPCMISMATCH", "RPC version wrong")),
            74 => Some(("EPROGUNAVAIL", "RPC prog. not avail")),
            75 => Some(("EPROGMISMATCH", "Program version wrong")),
            76 => Some(("EPROCUNAVAIL", "Bad procedure for program")),
            77 => Some(("ENOLCK", "No locks available")),
            78 => Some(("ENOSYS", "Function not implemented")),
            79 => Some(("EFTYPE", "Inappropriate file type or format")),
            80 => Some(("EAUTH", "Authentication error")),
            81 => Some(("ENEEDAUTH", "Need authenticator")),
            82 => Some(("EPWROFF", "Device power is off")),
            83 => Some(("EDEVERR", "Device error, e.g. paper out")),
            84 => Some(("EOVERFLOW", "Value too large to be stored in data type")),
            85 => Some(("EBADEXEC", "Bad executable")),
            86 => Some(("EBADARCH", "Bad CPU type in executable")),
            87 => Some(("ESHLIBVERS", "Shared library version mismatch")),
            88 => Some(("EBADMACHO", "Malformed Macho file")),
            89 => Some(("ECANCELED", "Operation canceled")),
            90 => Some(("EIDRM", "Identifier removed")),
            91 => Some(("ENOMSG", "No message of desired type")),
            92 => Some(("EILSEQ", "Illegal byte sequence")),
            93 => Some(("ENOATTR", "Attribute not found")),
            94 => Some(("EBADMSG", "Bad message")),
            95 => Some(("EMULTIHOP", "Reserved")),
            96 => Some(("ENODATA", "No message available on STREAM")),
            97 => Some(("ENOLINK", "Reserved")),
            98 => Some(("ENOSR", "No STREAM resources")),
            99 => Some(("ENOSTR", "Not a STREAM")),
            100 => Some(("EPROTO", "Protocol error")),
            101 => Some(("ETIME", "STREAM ioctl timeout")),
            102 => Some(("EOPNOTSUPP", "Operation not supported on socket")),
            103 => Some(("ENOPOLICY", "No such policy registered")),
            104 => Some(("ENOTRECOVERABLE", "State not recoverable")),
            105 => Some(("EOWNERDEAD", "Previous owner died")),
            106 => Some(("EQFULL", "Interface output queue is full")),
            256 => Some(("ECVCERORR", "kernel internal error")),
            512 => Some(("ECVPERORR", "kernel internal error")),
            _ => None,
        }
    }
}
