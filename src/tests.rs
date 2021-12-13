#[cfg(test)]
mod tests
{
    use crate::file_reader::{convert_lines_to_strings, read_lines};
    use crate::line_parser::get_initial_school;

    #[test]
    fn test_ocean_parsing()
    {
        const INPUT: &str = "Data/input_test.txt";
        let mut ocean = get_initial_school(convert_lines_to_strings(read_lines(INPUT)));
        assert_eq!(ocean.count(), 5);
        ocean.spend_one_day();
        assert_eq!(ocean.count(), 5);
        ocean.spend_one_day();
        assert_eq!(ocean.count(),6);
    }
    #[test]
    fn test_ocean_after_18_days()
    {
        const INPUT: &str = "Data/input_test.txt";
        let mut ocean = get_initial_school(convert_lines_to_strings(read_lines(INPUT)));
        while ocean.days_passed != 18 {
            ocean.spend_one_day();
        }
        assert_eq!(ocean.count(), 26 );
    }

    #[test]
    fn test_ocean_after_80_days()
    {
        const INPUT: &str = "Data/input_test.txt";
        let mut ocean = get_initial_school(convert_lines_to_strings(read_lines(INPUT)));
        while ocean.days_passed != 80{
            ocean.spend_one_day();
        }
        assert_eq!(ocean.count(), 5934 );
    }
}