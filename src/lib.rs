struct State {
    state: [[u64; 5]; 5]
}

impl State {
    fn absorb(&mut self, indx: usize, word: u64) {
        self.state[indx / 5][indx % 5] ^= word;
    }

    fn squeeze(&mut self, rate: usize, len: usize) -> Vec<u8> {
        let mut bytes = Vec::new();
        let x = rate / 64;
        let len_bytes = len / 8;

        while bytes.len() < len_bytes {
            for i in 0..x {
                bytes.extend_from_slice(&self.state[i / 5][i % 5].to_le_bytes());
            }

            permute(self);
        }

        return bytes[0..len_bytes].to_vec();
    }
}

impl Default for State {
    fn default () -> State {
        State {
            state: [[0; 5]; 5],
        }
    }
}

fn 
pad (message: &mut Vec<u8>, rate: usize) {

    let rate_bytes = rate / 8;

    if message.len() == rate_bytes -1 {
        message.push(0x06 | 0x80);
    } else {
        message.push(0x06);
        while (message.len() % rate_bytes) != rate_bytes - 1 {
            message.push(0x0);
        }
        message.push(0x80);
    }
}

fn
permute(state: &mut State) {
    for i in 0..24 {
        theta(state);
        rho_and_pi(state);
        chi(state);
        iota(state, i);
    }
}

fn 
theta (state: &mut State) {
    let mut c: [u64; 5] = [0; 5];
    let mut d: [u64; 5] = [0; 5];

    for i in 0..5 {
        // Calulate parities for each column
        c[i] = state.state[0][i] ^ state.state[1][i] ^ state.state[2][i] ^ state.state[3][i] ^ state.state[4][i];
    }

    for row in 0..5 {
        // Calculate XOR of column to the left and right
        // would use -1 but can't subtract here
        d[row] = c[(row + 4) % 5] ^ c[(row + 1) % 5].rotate_left(1);

        for col in 0..5 {
            // XOR state with corresponding elements from D array
            state.state[col][row] ^= d[row];
        }
    }
}

fn 
rho_and_pi (state: &mut State) {
    let rotation_offsets: [[u32; 5]; 5] = [
        [0,  1,  62, 28, 27],
        [36, 44, 6,  55, 20],
        [3,  10, 43, 25, 39],
        [41, 45, 15, 21, 8],
        [18, 2,  61, 56, 14]
      ];

    let mut B: [[u64; 5]; 5] = [[0; 5]; 5];

    for x in 0..5 {
        for y in 0..5 {
            //let x2 = (2 * y + 3 * x) % 5;
            let y2 = (3 * y + 2 * x) % 5;
            //let y = x;

            //let offset = ((x + y * 5) * (2 * x + 3 * y) % 5) % 64;
            // state.state[x2][y2] = state.state[x][y].rotate_left(rotation_offsets[x][y]);
            //B[y][(2 * x + 3*y) % 5] = state.state[x][y].rotate_left(offset.try_into().unwrap());
            
            //println!("\tnew x {} y2 {} -> {}", y, y2, rotation_offsets[x][y]);
            B[y2][y] = state.state[y][x].rotate_left(rotation_offsets[y][x]);
            // println!("\tstate[{}][{}][{}]->{} (x {}, y {}, x2 {}, y2 {}, offset x,y {}, state[x][y] {}", y, y2, y*5+y2, B[y][y2], x, y, y, y2, rotation_offsets[x][y], state.state[x][y]);
            //println!("{},{}\tstate[{}][{}]->{} (orig: {})\t(rot={})", y, x, y2, y, B[y2][y], state.state[y][x],rotation_offsets[y][x]);
        }
    }

    for x in 0..5 {
        for y in 0..5 {
            state.state[x][y] = B[x][y];
        }
    }
}

fn chi (state: &mut State) {
    for row in 0..5 {
        let mut temp: [u64; 5] = [0; 5];
        for col in 0..5 {
            temp[col] = state.state[row][col];
        }

        for col in 0..5 {
            state.state[row][col] ^= (!temp[(col + 1) % 5]) & temp[(col + 2) % 5];
        }
    }
}

fn iota (state: &mut State, round: usize) {
    let round_constants: [u64; 24] = [
        0x0000000000000001, 0x0000000000008082, 0x800000000000808A,
        0x8000000080008000, 0x000000000000808B, 0x0000000080000001,
        0x8000000080008081, 0x8000000000008009, 0x000000000000008A,
        0x0000000000000088, 0x0000000080008009, 0x000000008000000A,
        0x000000008000808B, 0x800000000000008B, 0x8000000000008089,
        0x8000000000008003, 0x8000000000008002, 0x8000000000000080,
        0x000000000000800A, 0x800000008000000A, 0x8000000080008081,
        0x8000000000008080, 0x0000000080000001, 0x8000000080008008
      ];

    state.state[0][0] ^= round_constants[round];
}

pub fn
hash (message: &mut Vec<u8>, size: usize) -> String {

    let mut state = State::default();

    let rate = match size {
        224 => 1152,
        256 => 1088,
        384 => 832,
        512 => 576,
        _ => 1088
    };

    pad (message, rate);

    for block in message.chunks(rate / 8) {

        let mut indx = 0;
        for chunk in block.chunks(8) {
            // Convert message byte chunks into a little-endian u64 integer
            let (b1, b2, b3, b4, b5, b6, b7, b8) 
              = (chunk[0] as u64, chunk[1] as u64, chunk[2] as u64, chunk[3] as u64, 
                  chunk[4] as u64, chunk[5] as u64, chunk[6] as u64, chunk[7] as u64);

             let word = (b8 << 56) | (b7 << 48) | (b6 << 40) | (b5 << 32) | (b4 << 24) | (b3 << 16) | (b2 << 8) | b1;

            state.absorb(indx, word);

            indx += 1;
        }

        permute(&mut state);
    }

    // Encode state into base 64
    return hex::encode(
        &state.squeeze(rate, size)
    ); 
}

/**
 * Convenience function for passing strings; converts given string to a Vector of u8 bytes for 
 * the hash() function.
 */
pub fn 
hash_string (message: &str, n: usize) -> String {
    let mut message_bytes = message.as_bytes().to_vec();
    return hash (&mut message_bytes, n);
}