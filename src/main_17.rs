task17
Vowel and Consonants Counter
Input a text and count the occurrences of vowels and consonant
Program Console Sample 1:
Enter text: QuickBrownFoxJumpsovertheDog
Vowels: 9
Consonants: 19


fn main() {
    println!("please enter something");
    let string: String = read!();
    //   println!("{}", data);
    let mut vowel: Vec<char> = Vec::new();
    let mut count = 0;
    let mut Vowel_Count = 0;
    let mut Consonant_Count = 0;
    for x in string.chars() {
        let y = (x.to_string()).parse::<char>().unwrap();
        vowel.push(y);
    }
    while count < vowel.len() {
        if vowel[count] == 'a'
            || vowel[count] == 'e'
            || vowel[count] == 'i'
            || vowel[count] == 'o'
            || vowel[count] == 'u'
        {
            Vowel_Count += 1;
        } else {
            Consonant_Count += 1;
        }  
        count +=1;
    }
     println!("VOWELS: {} Consonent: {}", Vowel_Count, Consonant_Count);
}

