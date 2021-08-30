mod hardware;
mod walk;

pub fn check() {
    walk::pos_a()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
