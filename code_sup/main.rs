fn main() {
    // generer des nombre premier
    //println!("{}", exponentiation_rapide(256,2,257));
    // partie 1
    let pub_key = generate_public_key_comment(5, 17, 5);
    let pri_key = generate_private_key_comment(5, 17, 5);
    let mut m = 10;
    println!("m={}\n",m);
    
    // partie 2
    m = encrypte_comment(m, pub_key);
    println!("m={}\n",m);
    
    // partie 3
    m = decrypte_comment(m, pri_key);
    println!("m={}\n",m);
    //test_probabiliste();
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
fn euclide_etendu_comment(a:isize, b:isize) -> (isize,isize) {
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
fn exponentiation_rapide_comment(m: usize, e: usize, n: usize) -> usize {
    let mut m:   usize = m;
    let mut e:   usize = e;
    let mut n:   usize = n;
    let mut mul: usize = 1;
    let mut res: usize = 0;
    println!("fast exponentiation");
    println!("  > begin while (e({}) > 0)", e);
    while e > 0 {
        println!("  >   begin loop");
        res = m % n;
        println!("  >     res({}) = m({}) % n({})", res, m, n);
        m = res * res;
        println!("  >     m({}) = res({}) * res({})", m, res, res);
        print!("  >     begin if (e({}) & 1 == 1) ", e);
        if e & 1 == 1 {
            println!("(true)");
            print!("  >       mul({}) *= res({}) => ", mul, res);
            mul *= res;
            println!("res({})", mul);
            print!("  >       mul({}) %= n({}) => ", mul, n);
            mul %= n;
            println!("res({})", mul);
            println!("  >     end if");
        } else {
            println!("(false)");
        }
        print!("  >       e({}) >>= 1 => ", e);
        e >>= 1;
        println!("e({})", e);
        println!("  >   end loop");
    }
    println!("  > end while");
    println!("returning result (mul({}))\n", mul);
    return mul;
}
fn generate_public_key(p: usize, q: usize, e: usize) -> (usize, usize) {
    let n: usize = p * q;
    return (n, e);
}
fn generate_public_key_comment(p: usize, q: usize, e: usize) -> (usize, usize) {
    println!("generating public key");
    let n: usize = p * q;
    println!("  > n({}) = p({}) * q({})", n, p, q);
    println!("returning result (n({}),e({}))\n", n, e);
    return (n, e);
}
fn generate_private_key(p: usize, q: usize, e: usize) -> (usize, usize) {
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
fn generate_private_key_comment(p: usize, q: usize, e: usize) -> (usize, usize) {
    println!("generating private key");
    let f: usize = (p - 1) * (q - 1);
    println!("  > f({}) = p({}-1) * q({}-1)", f, p, q);
    let n: usize = p * q;
    println!("  > n({}) = p({}) * q({})", n, p, q);
    let t = euclide_etendu_comment(e as isize, f as isize);
    println!("  > t({}, {}) = euclide_etendu(e({}), f({}))", t.0, t.1, e, f);
    let mut d: usize = 0;
    if t.0 >= 0 {
        d = t.0 as usize % f;
        println!("  > d({}) = t.0({}) % f({})", d, t.0, f);
    } else {
        d = f-((t.0).abs() % f as isize) as usize;
        println!("  > d({}) = f({})-((abs(t.0({}))) % f({}))", d, f, t.0, f);
    }
    println!("returning result (n({}),d({}))\n", n, d);
    return (n, d);
}
fn encrypte(m: usize, key: (usize, usize)) -> usize {
    return exponentiation_rapide(m, key.1, key.0);
}
fn encrypte_comment(m: usize, key: (usize, usize)) -> usize {
    return exponentiation_rapide_comment(m, key.1, key.0);
}
fn decrypte(y: usize, key: (usize, usize)) -> usize {
    return exponentiation_rapide(y, key.1, key.0);
}
fn decrypte_comment(y: usize, key: (usize, usize)) -> usize {
    return exponentiation_rapide_comment(y, key.1, key.0);
}



fn test_probabiliste() {
    let mut a:  usize = 97;
    let mut at: usize = a-1;
    let mut d:  usize = at;
    let mut s:  usize = 0;
    
    let mut k:  usize = 1;
    let mut r:  usize = 1;
    let mut x:  usize = 2;
    let mut y:  usize = 0;
    let mut f:  bool  = true;
    
    println!("  > begin while (d({}) & 1 == 0)", d);
    while d & 1 == 0 {
        println!("  >   begin loop");
        print!("  >     d({}) >>= 1 => ", d);
        d >>= 1;
        println!("d({})", d);
        print!("  >     s({}) += 1 => ", s);
        s += 1;
        println!("s({})", s);
        println!("  >   end loop");
    }
    println!("  > end while\n");
    
    println!("  > begin while (d({}) & 1 == 0)", d);
    while k < 40 {
        y = exponentiation_rapide(x, d, a);
        if y != 1 || y != at {
            while r <= s-1 {
                y = exponentiation_rapide(y, 2, a);
                if y == 1 {
                    k += 1;
                } else {
                    f = false;
                    break;
                }
                r += 1;
            }
        }
        if (r == s-1 && y != 1) || !f {
            println!("non premier !");
            break;
        } else {
            k += 1;
        }
    }
    println!("a  = {}", a );
    println!("at = {}", at);
    println!("d  = {}", d );
    println!("s  = {}", s );
    println!("k  = {}", k );
    println!("r  = {}", r );
    println!("x  = {}", x );
    println!("y  = {}", y );
    println!("f  = {}", f );
}

// alexandre.rosset@etu.hesge.ch
// jaQ*M6sk
