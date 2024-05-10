type Chars = Vec<char>;

pub struct Scanner {
    handle: std::io::Stdin,
}

pub trait Scannable {
    fn from_scan(s: String) -> Self;
}

macro_rules! impl_Scannable_for_numeical {
    ($($t:ty)*) => {
        $(
            impl Scannable for $t  {
                fn from_scan(s: String) -> Self {
                    s.as_str().parse::<$t>().unwrap()
                }
            }
        )*
    };
}
impl_Scannable_for_numeical!(usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64);

macro_rules! impl_Scannable_for_numeical_vector {
    ($($t:ty)*) => {
        $(
            impl Scannable for Vec<$t> {
                fn from_scan(s: String) -> Self {
                    s.split_whitespace()
                        .map(|i| i.parse::<$t>().unwrap())
                        .collect()
                }
            }
        )*
    };
}

impl_Scannable_for_numeical_vector!(usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64);

impl Scannable for Chars {
    fn from_scan(s: String) -> Self {
        s.chars().collect()
    }
}

impl Scannable for Vec<Chars> {
    fn from_scan(s: String) -> Self {
        s.split_whitespace()
            .map(|s| s.to_string().chars().collect::<Chars>())
            .collect()
    }
}

macro_rules! impl_for_tuple {
    ($($t: tt),*) => {
        impl<$($t),*> Scannable for ($($t),*)
        where
            $(
                $t: std::str::FromStr,
                <$t as std::str::FromStr>::Err: std::fmt::Debug,
            )*
        {
            fn from_scan(s: String) -> Self {
                let mut it = s.split_whitespace();
                (
                    $(
                        it.next().unwrap().parse::<$t>().unwrap(),
                    )*
                )
            }
        }
    };
}

impl_for_tuple!(A, B);
impl_for_tuple!(A, B, C);
impl_for_tuple!(A, B, C, D);

impl Scanner {
    pub fn new() -> Self {
        Scanner {
            handle: std::io::stdin(),
        }
    }
    pub fn read<T>(&self) -> T
    where
        T: Scannable,
    {
        let mut buf = String::new();
        self.handle.read_line(&mut buf).unwrap();
        <T as Scannable>::from_scan(
            buf.trim_end_matches('\n')
                .trim_end_matches('\r')
                .to_string(),
        )
    }
}
