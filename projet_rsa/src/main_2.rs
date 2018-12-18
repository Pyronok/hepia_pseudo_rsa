fn main() {
    let n: u64 = 2901984751;
    let e: u64 = 9103;
    let mut m: Vec<u64> = 
    vec![1780565330, 
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
     
     //recupere un tuple contenant les valeurs de p et q 
    let p_q = factoriser(n);
    let cle_privee = generer_cle_privee(p_q.0, p_q.1, e);
    
    for i in 0..m.len() {
        //dechiffrage de chaque champ du message
        m[i] = decrypter(m[i], cle_privee);
    }
    //traduction du code decrypter en caracteres lisibles
    match String::from_utf8(convertion_vec_byte(&mut m)) {
        Ok(s) => { println!("{}", s); }
        Err(e) => { println!("{}", e); }
    }
}

fn factoriser(n: u64) -> (u64, u64) {
    //b est un indice permettant de trouver un nombre premier
    let mut b: u64 = 2;
    let mut p: u64 = 0;
    let mut q: u64 = 0;
    let mut n_temp : u64 = n;
    while b != 0{
        //incrémente b jusqu'à trouver un nombre premier qui divise n
        while n_temp % b != 0 {
            b += 1;
        }
        //detecte l'autre partie 
        if n_temp == b {
            println!("p = {}", b);
            p = b;
            break;
        }
        println!("q = {}", b);
        q = b;
        //division de n par q
        n_temp /= b;
    }
    return (p, q);
}

fn generer_cle_privee(p: u64, q: u64, e: u64) -> (u64, u64) {
    let phi: u64 = (p - 1) * (q - 1);
    let n: u64 = p * q;
    let tuple_bezout = euclide_etendu(e as i64, phi as i64);
    println!("{:?}",tuple_bezout);
    //condition retournant le modulo de p  
    let d: u64 = if tuple_bezout.0 >= 0 {
        // si p est positif
        tuple_bezout.0 as u64 % phi
    } else {
        // si p est negatif
        phi-((tuple_bezout.0).abs() % phi as i64) as u64
    };
    //renvoie la cle privee [n,d]
    return (n, d);
}

fn euclide_etendu(a:i64, b:i64) -> (i64,i64) {
    let mut r:[i64;3] = [a,b,0];
    //bezout x et y
    let mut x:[i64;3] = [1,0,0];
    let mut y:[i64;3] = [0,1,0];
    //index des différentes tables pour 
    let mut i:[usize;3] = [0,1,2];
    loop {
        r[i[2]] = r[i[0]] % r[i[1]];
        //condition d'arrêt
        if r[i[2]] == 0 {
            return (x[i[1]], y[i[1]]);
        }

        //definition du quotient de la division euclidienne
        let q = r[i[0]] / r[i[1]];
        //utilisation du quotient afin de trouver le bézout x et y suivant
        x[i[2]] = x[i[0]] - q * x[i[1]];
        y[i[2]] = y[i[0]] - q * y[i[1]];
        //permute les index pour changer d'étape de transition
        // index permutés:
        // 0 --> 1
        // 1 --> 2
        // 2 --> 0
        for j in 0..i.len() {
            i[j] += 1;
            i[j] %= 3;
        }
    }
}

fn decrypter(y: u64, key: (u64, u64)) -> u64 {
    return exponentiation_rapide(y, key.1, key.0);
}

fn exponentiation_rapide(m: u64, e: u64, n: u64) -> u64 {
    let mut m:   u64 = m;
    let mut e:   u64 = e;
    let mut mul: u64 = 1;
    while e > 0 {
        let res = m % n;
        m = res * res;
        if e & 1 == 1 {
            mul *= res;
            mul %= n;
        }
        e >>= 1;
    }
    return mul;
}

fn convertion_vec_byte(m : &mut Vec<u64>) -> Vec<u8> {
    let mut message = Vec::new();
    let mut mult = 0;
    let mut tmp_byte = 0;
    for i in 0..m.len() {
        while m[i] > 0 {
            tmp_byte += ((m[i] & 1)) * 2_u64.pow(mult);
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
