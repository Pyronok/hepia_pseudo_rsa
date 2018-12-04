fn main() {
    let n: usize = 2901984751;
    let e: usize = 9103;
    let mut m: [usize; 31] = 
    [1780565330, 
     1418927598, 
     543482106, 
     729172139, 
     111350267, 
     2866131698, 
     353182206, 
     2750311025, 
     1740400630, 
     2336243297, 
     570711647, 
     2642624210, 
     1208279921, 
     2741398971, 
     381511738, 
     1701929578, 
     2875813324, 
     1677894499, 
     1797737510, 
     2901354249, 
     1627727243, 
     762227604, 
     756639409, 
     600478187, 
     2152502192, 
     10636100, 
     2133402040, 
     1181530544, 
     46461495, 
     1681846270, 
     432128454];
     
    let p_q = factoriser(n);
    let cle_privee = generer_cle_privee(p_q.0, p_q.1, e);
    
    for i in 0..m.len() {
        m[i] = decrypter(m[i], cle_privee);
    }
    
    match String::from_utf8(convertion_vec_byte(&mut m)) {
        Ok(S) => { println!("{}", S); }
        Err(E) => { println!("{}", E); }
    }
}

fn factoriser(n: usize) -> (usize, usize) {
    let mut b: usize = 2;
    let mut p: usize = 0;
    let mut q: usize = 0;
    let mut nt : usize = n;
    while b != 0 {
        while nt % b != 0 {
            b += 1;
        }
        if nt / b == 1 {
            //println!("p = {}", b);
            p = b;
            break;
        }
        //println!("q = {}", b);
        q = b;
        nt /= b;
    }
    return (p, q);
}

fn generer_cle_privee(p: usize, q: usize, e: usize) -> (usize, usize) {
    let f: usize = (p - 1) * (q - 1);
    let n: usize = p * q;
    let t = euclide_etendu(e as isize, f as isize);
    let mut d: usize = 0;
    if t.0 >= 0 {
        d = t.0 as usize % f;
    } else {
        d = f-((t.0).abs() % f as isize) as usize;
    }
    return (n, d);
}

fn euclide_etendu(a:isize, b:isize) -> (isize,isize) {
    let mut r:[isize;3] = [a,b,0];
    let mut x:[isize;3] = [1,0,0];
    let mut y:[isize;3] = [0,1,0];
    let mut i:[usize;3] = [0,1,2];
    let mut q = 0;
    let mut j = 0;
    loop {
        r[i[2]] = r[i[0]] % r[i[1]];
        q = r[i[0]] / r[i[1]];
        if r[i[2]] == 0 {
            x[i[2]] = x[i[1]];
            y[i[2]] = y[i[1]];
            break;
        }
        x[i[2]] = x[i[0]] - q * x[i[1]];
        y[i[2]] = y[i[0]] - q * y[i[1]];
        for j in 0..3 {
            i[j] += 1;
            i[j] %= 3;
        }
        
        j+=1;
    }
    return (x[i[2]], y[i[2]]);
}

fn decrypter(y: usize, key: (usize, usize)) -> usize {
    return exponentiation_rapide(y, key.1, key.0);
}

fn exponentiation_rapide(m: usize, e: usize, n: usize) -> usize {
    let mut m:   usize = m;
    let mut e:   usize = e;
    let mut n:   usize = n;
    let mut mul: usize = 1;
    let mut res: usize = 0;
    while e > 0 {
        res = m % n;
        m = res * res;
        if e & 1 == 1 {
            mul *= res;
            mul %= n;
        }
        e >>= 1;
    }
    return mul;
}

fn convertion_vec_byte(m : &mut [usize; 31]) -> Vec<u8> {
    let mut message = Vec::new();
    let mut mult = 0;
    let mut tmp_byte = 0;
    for i in 0..m.len() {
        while m[i] > 0 {
            tmp_byte += ((m[i] & 1)) * 2_usize.pow(mult);
            m[i] >>= 1;
            mult += 1;
            if mult == 8 || m[i] == 0 {
                mult = 0;
                message.push(tmp_byte as u8);
                tmp_byte = 0;
            }
        }
        tmp_byte = 0;
        mult = 0;
    }
    return message;
}
