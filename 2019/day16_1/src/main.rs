
type Nums = Vec<i32>;

fn str_to_vec(input: &str) -> Nums {
    input.trim().chars().map(|x| x.to_digit(10).unwrap() as i32).collect()
}

fn fft_phase(input: Nums) -> Nums {
    let base_pattern = vec![0, 1, 0, -1];
    let mut output = Vec::new();
    for n in 1..=input.len() {
        let mut pattern = Vec::new();
        for p in base_pattern.iter() {
            pattern.resize(pattern.len()+n, p);
        }
        pattern.rotate_left(1);
        let mut sum: i32 = 0;
        for (i,val) in input.iter().enumerate() {
            sum += val * pattern[i % pattern.len()];
        }
        output.push(sum.abs() % 10);
    }
    output
}

fn fft_phase_loop(input: Nums, num_phases: usize) -> Nums {
    let mut output = input;
    for _ in 0..num_phases {
        output = fft_phase(output);
    }
    output
}

fn main() {
    let input = str_to_vec("59796737047664322543488505082147966997246465580805791578417462788780740484409625674676660947541571448910007002821454068945653911486140823168233915285229075374000888029977800341663586046622003620770361738270014246730936046471831804308263177331723460787712423587453725840042234550299991238029307205348958992794024402253747340630378944672300874691478631846617861255015770298699407254311889484508545861264449878984624330324228278057377313029802505376260196904213746281830214352337622013473019245081834854781277565706545720492282616488950731291974328672252657631353765496979142830459889682475397686651923318015627694176893643969864689257620026916615305397");
    let output = fft_phase_loop(input, 100);
    print!("first 8 digits: ");
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
}