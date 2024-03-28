#[cfg(test)]
mod tests {
    use crate::{phonology::vowel::Vowel, V};

    #[test]
    fn it_works() {
        let vowel : Vowel = V!();
    }
}