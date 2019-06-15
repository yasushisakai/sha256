const U32_MAX:u64 = u32::max_value() as u64;

#[allow(dead_code)]
const K:[u32;64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2 
];

#[allow(dead_code)]
const H:[u32; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19 
];



pub fn padding(m: &str) -> Vec<u32> {

    let chars = m.chars();

    let mut a = Vec::with_capacity(m.len());

    for (_, c) in chars.enumerate() {
        a.push(u32::from(c));
    }

    println!("{:?}", &a);

    padding_u32(&a)
}



// TODO: out Result?
pub fn padding_u32(m: &[u32]) -> Vec<u32> {

    let mut vec = Vec::new();
    let str_len = m.len();
    
    let non_zeros = (str_len * 8) + 1 + 64; // in binary
    let n = (non_zeros / 512) + 1;
    let zeros = n * 512 - non_zeros; // num of zeros in binary;

    let mut w:u32 = 0;

    let mut i = 0;
    for c in m.iter() {
        w |= c << (3 - i % 4) * 8;
        
        if i % 4 == 3 {
            vec.push(w);
            w = 0;
        }

        i += 1;
    }
    
    // add one to the end of the data
    let tail = (3 - str_len % 4) * 8 + 7; 
    w |= 1 << tail;
    vec.push(w);
    
    let zero_vec = vec![0; (zeros - tail) / 32];
    vec.extend(zero_vec);

    // add the number of bits using last 64bits (u32 x 2)
    let len_64 = (str_len as u64) * 8;
    let high = (len_64 >> 32) as u32;
    let low = len_64 as u32; // just gets the lower 32 bits

    vec.push(high);
    vec.push(low);

    vec
}

// TODO: maybe its efficient to make the 2d matrix inside padding
pub fn parse(v :&Vec<u32>) -> Vec<Vec<u32>>  {
    let n = v.len() /  16; // 512 bits / 32 bits
    println!("{}", n);
    // get n
    
    let mut whole: Vec<Vec<u32>> = Vec::new();

    for i in 0..n {
        whole.push(Vec::new());
        for j in 0..16 {
            whole[i].push(v[i * 16 + j]);
        }
    }
   whole 
}

fn ch (x: u32, y: u32, z:u32) -> u32 {
    (x & y) ^ (!x & z)
}

fn maj (x: u32, y: u32, z:u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

fn sig_0 (x: u32) -> u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}

fn sig_1 (x: u32) -> u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

fn e_0 (x:u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ x >> 3
}

fn e_1 (x:u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ x >> 10
}


/// add_modular!
/// macro to circumvent the u32 overflow panic,
/// modular arithmetic
macro_rules! add_modular {
    ($ ($x:expr), *) => {
        {
        let mut n: u64 = 0;
        $(
            // n = (n + ($x as u64)) % (u32::max_value() as u64 + 1);
            n = (n + ($x as u64)) & U32_MAX; // bit masking
            )*
        n as u32
        }
    };
}


pub fn hash(message: &str) -> [u32;8] {
    let m = parse(&padding(&message));

    let mut h1 = H[0] as u32;
    let mut h2 = H[1] as u32;
    let mut h3 = H[2] as u32;
    let mut h4 = H[3] as u32;
    let mut h5 = H[4] as u32;
    let mut h6 = H[5] as u32;
    let mut h7 = H[6] as u32;
    let mut h8 = H[7] as u32;

    for b in m {

        let mut w = [0 as u32; 64];

        for t in 0..16 {
            w[t] = b[t];
        }

        for t in 16..64 {
            w[t] = add_modular!(e_1(w[t-2]), w[t-7], e_0(w[t-15]), w[t-16]);
        }

        let mut a = h1;
        let mut b = h2;
        let mut c = h3;
        let mut d = h4;
        let mut e = h5;
        let mut f = h6;
        let mut g = h7;
        let mut h = h8;
        
        for t in 0..64 {
            let t1 = add_modular!(h, sig_1(e), ch(e, f, g), K[t], w[t]);
            let t2 = add_modular!(sig_0(a), maj(a, b, c));

            h = g;
            g = f;
            f = e;
            e = add_modular!(d, t1);
            d = c;
            c = b;
            b = a;
            a = add_modular!(t1,t2);
        }

        h1 = add_modular!(h1, a);
        h2 = add_modular!(h2, b);
        h3 = add_modular!(h3, c);
        h4 = add_modular!(h4, d);
        h5 = add_modular!(h5, e);
        h6 = add_modular!(h6, f);
        h7 = add_modular!(h7, g);
        h8 = add_modular!(h8, h);
    }

     [h1, h2, h3, h4, h5, h6, h7, h8]
}


pub fn format_hash(h: &[u32;8]) -> String {
    let mut o = String::from("");
    for i in 0..8 {
        let raw = format!("{:08x}", &h[i]);
        o.push_str(&raw);
    }
    o
}
