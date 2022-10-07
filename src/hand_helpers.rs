use crate::prelude::*;

const CARD_HAND_WIDTH: f32 = WINDOW_WIDTH as f32 / 2.0;

pub fn calculate_card_center_x(idx: i32, hand_size: i32) -> f32 {
    let center_x = WINDOW_WIDTH as f32 * 0.5;
    let left_x = center_x - CARD_HAND_WIDTH * 0.5;
    let division_size = CARD_HAND_WIDTH / hand_size as f32;
    left_x + (division_size * 0.5) + (division_size * idx as f32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_card_hand() {
        let expected = WINDOW_WIDTH as f32 * 0.5;

        assert_eq!(calculate_card_center_x(0, 1), expected);
    }

    #[test]
    fn test_two_card_hand() {
        let first_expected = (WINDOW_WIDTH as f32 * 0.5) - CARD_HAND_WIDTH * 0.25;
        let second_expected = (WINDOW_WIDTH as f32 * 0.5) + CARD_HAND_WIDTH * 0.25;

        assert_eq!(calculate_card_center_x(0, 2), first_expected);
        assert_eq!(calculate_card_center_x(1, 2), second_expected);
    }
}
