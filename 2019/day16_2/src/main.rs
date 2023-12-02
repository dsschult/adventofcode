type Num = u8;
type Nums = Vec<Num>;

fn str_to_vec(input: &str) -> Nums {
    input.trim().chars().map(|x| x.to_digit(10).unwrap() as Num).collect()
}

fn fft_phase(input: Nums) -> Nums {
    //let base_pattern = [0, 1, 0, -1];
    let mut output: Vec<i64> = vec![0; input.len()];
    for (i,val) in input.iter().enumerate() {
        for n in 0..input.len() {
            let p = match ((i+1) as f32 / (n+1) as f32) as usize % 4 {
                0 => 0i64,
                1 => 1i64,
                2 => 0i64,
                3 => -1i64,
                _ => panic!("should never get here"),
            };
            //output[n] += (*val as i8 * base_pattern[((i+1) as f32 / (n+1) as f32) as usize % 4]) as i64;
            output[n] += *val as i64 * p;
        }
    }
    output.iter().map(|x| (x.abs() % 10) as Num).collect()
}

fn fft_phase_loop(input: Nums, num_phases: usize) -> Nums {
    let mut output = input;
    for i in 0..num_phases {
        println!("running fft pass {}", i+1);
        output = fft_phase(output);
    }
    output
}

fn fft_partial_sums(input: Nums, num_phases: usize) -> Nums {
    let mut output = input;
    for _ in 0..num_phases {
        for i in (0..output.len()-1).rev() {
            output[i] = (output[i] + output[i+1]) % 10;
        }
    }
    output
}

fn repeat_vec(input: Nums, reps: usize) -> Nums {
    let mut output = vec![0; input.len()*reps];
    for n in 0..reps {
        for (i,v) in input.iter().enumerate() {
            output[i + n*input.len()] = *v;
        }
    }
    output
}

fn get_offset(buf: &Nums) -> usize {
    let mut ret = 0;
    for n in buf[..7].iter() {
        ret = ret*10+ (*n as usize);
    }
    ret
}

fn main() {
    let input = str_to_vec("59796737047664322543488505082147966997246465580805791578417462788780740484409625674676660947541571448910007002821454068945653911486140823168233915285229075374000888029977800341663586046622003620770361738270014246730936046471831804308263177331723460787712423587453725840042234550299991238029307205348958992794024402253747340630378944672300874691478631846617861255015770298699407254311889484508545861264449878984624330324228278057377313029802505376260196904213746281830214352337622013473019245081834854781277565706545720492282616488950731291974328672252657631353765496979142830459889682475397686651923318015627694176893643969864689257620026916615305397");
    let mut input = repeat_vec(input, 10000);
    let offset = get_offset(&input);
    println!("offset: {}", offset);
    let input = input.drain(offset..).collect();
    let output = fft_partial_sums(input, 100);
    
    print!("val: ");
    for x in output[..8].iter() {
        print!("{}", x);
    }
    println!("");
}

mod tests {
    use super::*;

    #[test]
    fn test_str_to_vec() {
        let input = "12345";
        let output = str_to_vec(input);
        assert_eq!(output, vec![1,2,3,4,5]);
    }

    #[test]
    fn test_repeat_vec() {
        let input = "12345";
        let output = repeat_vec(str_to_vec(input),10000);
        assert_eq!(output[..20], vec![1,2,3,4,5,1,2,3,4,5,1,2,3,4,5,1,2,3,4,5][..]);
    }

    #[test]
    fn test_get_offset() {
        let input = vec![1,2,3,4,5,6,7,8,9,1,2];
        let output = get_offset(&input);
        assert_eq!(output, 1234567);
    }

    #[test]
    fn test_fft_phase() {
        let input = str_to_vec("12345678");
        let output = fft_phase(input);
        assert_eq!(output, vec![4,8,2,2,6,1,5,8]);
        let output = fft_phase(output);
        assert_eq!(output, vec![3,4,0,4,0,4,3,8]);
        let output = fft_phase(output);
        assert_eq!(output, vec![0,3,4,1,5,5,1,8]);
        let output = fft_phase(output);
        assert_eq!(output, vec![0,1,0,2,9,4,9,8]);
    }

    #[test]
    fn test_fft_phase_a() {
        let input = str_to_vec("80871224585914546619083218645595");
        let output = fft_phase_loop(input, 100);
        assert_eq!(output[..8], vec![2,4,1,7,6,1,7,6][..]);
    }

    #[test]
    fn test_fft_phase_b() {
        let input = str_to_vec("19617804207202209144916044189917");
        let output = fft_phase_loop(input, 100);
        assert_eq!(output[..8], vec![7,3,7,4,5,4,1,8][..]);
    }

    #[test]
    fn test_fft_phase_c() {
        let input = str_to_vec("69317163492948606335995924319873");
        let output = fft_phase_loop(input, 100);
        assert_eq!(output[..8], vec![5,2,4,3,2,1,3,3][..]);
    }

    #[test]
    fn test_fft_phase_2a() {
        let mut input = repeat_vec(str_to_vec("03036732577212944063491565474664"),10000);
        let offset = get_offset(&input);
        assert_eq!(offset, 0303673);
        let input = input.drain(offset..).collect();
        let output = fft_partial_sums(input, 100);
        assert_eq!(output[..8], vec![8,4,4,6,2,0,2,6][..]);
    }

    #[test]
    fn test_fft_phase_2b() {
        let mut input = repeat_vec(str_to_vec("02935109699940807407585447034323"),10000);
        let offset = get_offset(&input);
        assert_eq!(offset, 0293510);
        let input = input.drain(offset..).collect();
        let output = fft_partial_sums(input, 100);
        assert_eq!(output[..8], vec![7,8,7,2,5,2,7,0][..]);
    }

    #[test]
    fn test_fft_phase_2c() {
        let mut input = repeat_vec(str_to_vec("03081770884921959731165446850517"),10000);
        let offset = get_offset(&input);
        assert_eq!(offset, 0308177);
        let input = input.drain(offset..).collect();
        let output = fft_partial_sums(input, 100);
        assert_eq!(output[..8], vec![5,3,5,5,3,7,3,1][..]);
    }
}