/// Galois Field GF(2^8) arithmetic
pub struct GF {
    pub exp: [u8; 512],
    pub log: [usize; 256],
}

impl GF {
    pub fn new() -> Self {
        let mut exp = [0; 512];
        let mut log = [0; 256];
        let mut x = 1;
        for i in 0..255 {
            exp[i] = x as u8;
            exp[i + 255] = x as u8;
            log[x] = i;
            x <<= 1;
            if x & 0x100 != 0 {
                x ^= 0x11D;
            }
        }
        GF { exp, log }
    }

    pub fn add(&self, a: u8, b: u8) -> u8 {
        a ^ b
    }

    pub fn mul(&self, a: u8, b: u8) -> u8 {
        if a == 0 || b == 0 {
            0
        } else {
            self.exp[self.log[a as usize] + self.log[b as usize]]
        }
    }

    pub fn inverse(&self, a: u8) -> u8 {
        if a == 0 {
            panic!("GF divide by zero");
        }
        self.exp[255 - self.log[a as usize]]
    }

    pub fn div(&self, a: u8, b: u8) -> u8 {
        if a == 0 {
            0
        } else {
            self.mul(a, self.inverse(b))
        }
    }

    pub fn poly_scale(&self, p: &[u8], scalar: u8) -> Vec<u8> {
        p.iter().map(|&c| self.mul(c, scalar)).collect()
    }

    pub fn poly_add(&self, p: &[u8], q: &[u8]) -> Vec<u8> {
        let max_len = std::cmp::max(p.len(), q.len());
        let mut res = vec![0; max_len];
        for i in 0..p.len() {
            res[i] ^= p[i];
        }
        for i in 0..q.len() {
            res[i] ^= q[i];
        }
        res
    }

    pub fn poly_mul(&self, p: &[u8], q: &[u8]) -> Vec<u8> {
        let mut result = vec![0; p.len() + q.len() - 1];
        for (i, &pv) in p.iter().enumerate() {
            for (j, &qv) in q.iter().enumerate() {
                result[i + j] = self.add(result[i + j], self.mul(pv, qv));
            }
        }
        result
    }

    pub fn poly_remainder(&self, dividend: &[u8], divisor: &[u8], verbose: bool) -> Vec<u8> {
        let mut msg = dividend.to_vec();
        if verbose {
            println!("\n[VERBOSE] --- Polynomial Division ---");
            println!("[VERBOSE] Dividend: {:?}", msg);
            println!("[VERBOSE] Divisor:  {:?}", divisor);
        }
        for i in 0..=(dividend.len() - divisor.len()) {
            let coef = msg[i];
            if coef != 0 {
                for j in 0..divisor.len() {
                    msg[i + j] = self.add(msg[i + j], self.mul(divisor[j], coef));
                }
            }
        }
        let remainder = msg[dividend.len() - divisor.len() + 1..].to_vec();
        if verbose {
            println!("[VERBOSE] Remainder: {:?}", remainder);
        }
        remainder
    }

    pub fn generator_poly(&self, ecc_len: usize) -> Vec<u8> {
        let mut g_poly = vec![1];
        for i in 0..ecc_len {
            let root = vec![1, self.exp[i]];
            g_poly = self.poly_mul(&g_poly, &root);
        }
        g_poly
    }

    pub fn poly_eval(&self, poly: &[u8], x: u8) -> u8 {
        let mut y = poly[0];
        for i in 1..poly.len() {
            y = self.add(self.mul(y, x), poly[i]);
        }
        y
    }

    pub fn poly_eval_lsb(&self, poly: &[u8], x: u8) -> u8 {
        let mut y = 0;
        let mut power = 1;
        for &coef in poly {
            if coef != 0 {
                y = self.add(y, self.mul(coef, power));
            }
            power = self.mul(power, x);
        }
        y
    }

    pub fn poly_deriv_lsb(&self, poly: &[u8]) -> Vec<u8> {
        let mut deriv = vec![0; poly.len().saturating_sub(1)];
        for i in 1..poly.len() {
            if i % 2 != 0 {
                deriv[i - 1] = poly[i];
            }
        }
        deriv
    }

    pub fn berlekamp_massey(&self, syndromes: &[u8]) -> Vec<u8> {
        let mut c = vec![1];
        let mut b = vec![1];
        let mut l = 0;
        let mut m = 1;
        let mut b_scale = 1;

        for i in 0..syndromes.len() {
            let mut delta = syndromes[i];
            for j in 1..=l {
                if i >= j && c.len() > j {
                    delta = self.add(delta, self.mul(c[j], syndromes[i - j]));
                }
            }

            if delta != 0 {
                let mut shifted_b = vec![0; m];
                shifted_b.extend(&b);
                let term = self.poly_scale(&shifted_b, self.div(delta, b_scale));
                let new_c = self.poly_add(&c, &term);

                if 2 * l <= i {
                    l = i + 1 - l;
                    b = c;
                    b_scale = delta;
                    m = 1;
                } else {
                    m += 1;
                }
                c = new_c;
            } else {
                m += 1;
            }
        }
        c
    }
}

pub struct ReedSolomon {
    gf: GF,
    pub ecc_len: usize,
}

impl ReedSolomon {
    pub fn new(ecc_len: usize) -> Self {
        ReedSolomon {
            gf: GF::new(),
            ecc_len,
        }
    }

    pub fn encode(&self, message: &[u8], verbose: bool) -> Vec<u8> {
        let g_poly = self.gf.generator_poly(self.ecc_len);
        let mut padded = message.to_vec();
        padded.extend(vec![0; self.ecc_len]);
        let remainder = self.gf.poly_remainder(&padded, &g_poly, verbose);
        let mut codeword = message.to_vec();
        codeword.extend(remainder);
        codeword
    }

    pub fn calc_syndromes(&self, codeword: &[u8]) -> Vec<u8> {
        let mut syndromes = vec![0; self.ecc_len];
        for i in 0..self.ecc_len {
            syndromes[i] = self.gf.poly_eval(codeword, self.gf.exp[i]);
        }
        syndromes
    }

    pub fn correct_errors(&self, codeword: &mut [u8], verbose: bool) -> Result<(), String> {
        let syndromes = self.calc_syndromes(codeword);
        if syndromes.iter().all(|&s| s == 0) {
            if verbose {
                println!("[VERBOSE] Syndromes are zero. No correction needed.");
            }
            return Ok(());
        }

        let locator = self.gf.berlekamp_massey(&syndromes);
        if verbose {
            println!("[VERBOSE] Error Locator Poly (Lambda): {:?}", locator);
        }

        let mut error_degrees = Vec::new();
        let num_errors = locator.len() - 1;
        let n = codeword.len();

        for j in 0..n {
            let root_inv = self.gf.exp[(255 - (j % 255)) % 255]; // alpha^{-j}
            if self.gf.poly_eval_lsb(&locator, root_inv) == 0 {
                error_degrees.push(j);
            }
        }

        if verbose {
            println!("[VERBOSE] Error Degrees Found: {:?}", error_degrees);
        }

        if error_degrees.len() != num_errors {
            return Err(
                "Chien search failed to find correct number of roots. Unrecoverable.".into(),
            );
        }

        let omega_full = self.gf.poly_mul(&syndromes, &locator);
        let omega: Vec<u8> = omega_full.into_iter().take(self.ecc_len).collect();
        if verbose {
            println!("[VERBOSE] Error Evaluator Poly (Omega): {:?}", omega);
        }

        let lambda_prime = self.gf.poly_deriv_lsb(&locator);

        for &j in &error_degrees {
            let root_inv = self.gf.exp[(255 - (j % 255)) % 255];
            let num = self.gf.poly_eval_lsb(&omega, root_inv);
            let den = self
                .gf
                .mul(root_inv, self.gf.poly_eval_lsb(&lambda_prime, root_inv));

            if den == 0 {
                return Err("Forney Algorithm encountered division by zero.".into());
            }
            let y = self.gf.div(num, den);

            let idx = n - 1 - j;
            if verbose {
                println!("[VERBOSE] Correcting index {} with value {:02X}", idx, y);
            }
            codeword[idx] = self.gf.add(codeword[idx], y);
        }

        let verify_syndromes = self.calc_syndromes(codeword);
        if verify_syndromes.iter().all(|&s| s == 0) {
            Ok(())
        } else {
            Err("Correction applied, but codeword is still invalid.".into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gf_add_is_xor() {
        let gf = GF::new();
        assert_eq!(gf.add(0xAB, 0xCD), 0xAB ^ 0xCD);
        assert_eq!(gf.add(0xFF, 0xFF), 0x00); // a + a = 0 in GF(2)
        assert_eq!(gf.add(0x00, 0x42), 0x42);
    }

    #[test]
    fn gf_mul_commutativity() {
        let gf = GF::new();
        for a in [0x00u8, 0x01, 0x02, 0x10, 0x7F, 0xFF] {
            for b in [0x00u8, 0x01, 0x03, 0x20, 0x80, 0xFE] {
                assert_eq!(
                    gf.mul(a, b),
                    gf.mul(b, a),
                    "mul not commutative for {a:#x}, {b:#x}"
                );
            }
        }
    }

    #[test]
    fn gf_mul_identity_and_zero() {
        let gf = GF::new();
        for a in 0u8..=255 {
            assert_eq!(gf.mul(a, 1), a, "mul by 1 should be identity for {a:#x}");
            assert_eq!(gf.mul(a, 0), 0, "mul by 0 should be 0 for {a:#x}");
        }
    }

    #[test]
    fn gf_inverse_roundtrip() {
        let gf = GF::new();
        for a in 1u8..=255 {
            assert_eq!(
                gf.mul(a, gf.inverse(a)),
                1,
                "a * inv(a) should be 1 for {a:#x}"
            );
        }
    }

    #[test]
    fn gf_div_roundtrip() {
        let gf = GF::new();
        for a in [0x01u8, 0x02, 0x10, 0x7F, 0xFF] {
            for b in [0x01u8, 0x03, 0x20, 0x80, 0xFE] {
                let q = gf.div(a, b);
                assert_eq!(
                    gf.mul(q, b),
                    a,
                    "div/mul roundtrip failed for {a:#x}/{b:#x}"
                );
            }
        }
    }

    fn roundtrip(msg: &[u8], ecc_len: usize) {
        let rs = ReedSolomon::new(ecc_len);
        let codeword = rs.encode(msg, false);
        assert_eq!(codeword.len(), msg.len() + ecc_len);

        let mut received = codeword.clone();
        rs.correct_errors(&mut received, false)
            .expect("clean codeword should need no correction");
        assert_eq!(&received[..msg.len()], msg, "decoded message mismatch");
    }

    #[test]
    fn encode_decode_roundtrip_short() {
        roundtrip(b"Hello", 6);
    }

    #[test]
    fn encode_decode_roundtrip_long() {
        roundtrip(b"Reed-Solomon error correction", 10);
    }

    #[test]
    fn encode_decode_roundtrip_binary_data() {
        let msg: Vec<u8> = (0u8..=50).collect();
        roundtrip(&msg, 8);
    }

    #[test]
    fn encode_decode_roundtrip_single_byte() {
        roundtrip(&[0xAB], 4);
    }

    #[test]
    fn syndromes_zero_for_valid_codeword() {
        let rs = ReedSolomon::new(6);
        let codeword = rs.encode(b"TestData", false);
        let syndromes = rs.calc_syndromes(&codeword);
        assert!(
            syndromes.iter().all(|&s| s == 0),
            "syndromes must be zero for a valid codeword"
        );
    }

    #[test]
    fn syndromes_nonzero_after_corruption() {
        let rs = ReedSolomon::new(6);
        let mut codeword = rs.encode(b"TestData", false);
        codeword[0] ^= 0xFF;
        let syndromes = rs.calc_syndromes(&codeword);
        assert!(
            !syndromes.iter().all(|&s| s == 0),
            "syndromes must be non-zero after corruption"
        );
    }

    fn correct_and_check(msg: &[u8], ecc_len: usize, error_positions: &[(usize, u8)]) {
        let rs = ReedSolomon::new(ecc_len);
        let mut codeword = rs.encode(msg, false);
        for &(pos, mask) in error_positions {
            codeword[pos] ^= mask;
        }
        rs.correct_errors(&mut codeword, false)
            .unwrap_or_else(|e| panic!("correction failed with {error_positions:?}: {e}"));
        assert_eq!(
            &codeword[..msg.len()],
            msg,
            "message not restored after correcting {error_positions:?}"
        );
    }

    #[test]
    fn correct_one_error_in_data() {
        correct_and_check(b"Hello", 6, &[(0, 0xFF)]);
    }

    #[test]
    fn correct_one_error_in_ecc() {
        let msg = b"Hello";
        correct_and_check(msg, 6, &[(msg.len() + 1, 0xAB)]);
    }

    #[test]
    fn correct_max_errors_ecc6() {
        // ecc_len = 6 can correct up to 3 errors
        correct_and_check(b"Hello!!", 6, &[(0, 0x11), (2, 0xAA), (4, 0x55)]);
    }

    #[test]
    fn correct_max_errors_ecc8() {
        // ecc_len = 8 can correct up to 4 errors
        correct_and_check(
            b"ReedSolomon",
            8,
            &[(0, 0x01), (2, 0x02), (5, 0x04), (8, 0x08)],
        );
    }

    #[test]
    fn too_many_errors_returns_err() {
        let rs = ReedSolomon::new(6); // capacity = 3 errors
        let mut codeword = rs.encode(b"Hello", false);
        codeword[0] ^= 0x01;
        codeword[1] ^= 0x02;
        codeword[2] ^= 0x04;
        codeword[3] ^= 0x08;
        assert!(
            rs.correct_errors(&mut codeword, false).is_err(),
            "should fail with too many errors"
        );
    }
}
