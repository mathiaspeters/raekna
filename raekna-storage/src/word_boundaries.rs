use raekna_common::BoundaryPriority;

/// Returns the indices of the word boundaries from an origin point.
///
/// There are three classes of characters that create words when the appear in sequence:
/// 1. Alphanumeric and underscores
/// 2. Whitespace
/// 3. Everything else
///
/// If the origin is on the border between two words the above numbering shows the priority of different types of words.
///
/// The following string, for example, contains four separate words:
/// `abc   ,.-de_f`
///
/// This function does no specific handling of newlines since for the use case strings cannot have newlines.
pub fn find_word_boundaries(
    input: &str,
    origin: usize,
    priority: BoundaryPriority,
) -> Option<(usize, usize)> {
    if input.is_empty() || origin > input.len() {
        return None;
    }
    let sequences = generate_sequences(input);
    find_sequence(&sequences, origin, priority).map(CharSequence::boundaries)
}

fn generate_sequences(input: &str) -> Vec<CharSequence> {
    let mut sequences = Sequences::new();
    input.chars().into_iter().enumerate().for_each(|(i, c)| {
        let ctype = if c.is_alphanumeric() || c == '_' {
            SequenceType::Word
        } else if c.is_whitespace() {
            SequenceType::Whitespace
        } else {
            SequenceType::Other
        };
        sequences.increment(ctype, i);
    });
    sequences.finish()
}

fn find_sequence(
    sequences: &[CharSequence],
    origin: usize,
    priority: BoundaryPriority,
) -> Option<CharSequence> {
    let mut seq = None;
    let mut seq_index = None;
    for (i, cs) in sequences.iter().enumerate() {
        if cs.start > origin {
            break;
        } else if cs.start <= origin && cs.end >= origin {
            match priority {
                BoundaryPriority::None => {
                    match (cs.seq_type, seq.map(|cs: CharSequence| cs.seq_type)) {
                        (
                            SequenceType::Word,
                            Some(SequenceType::Whitespace | SequenceType::Other),
                        )
                        | (SequenceType::Whitespace, Some(SequenceType::Other))
                        | (_, None) => {
                            seq = Some(*cs);
                        }
                        _ => {}
                    }
                }
                BoundaryPriority::Left => {
                    seq = Some(*cs);
                    seq_index = Some(i);
                    break;
                }
                BoundaryPriority::Right => {
                    seq = Some(*cs);
                    seq_index = Some(i);
                }
            }
        }
    }
    match seq_index {
        Some(i) => match priority {
            BoundaryPriority::Left if i == 0 => seq,
            BoundaryPriority::Left => {
                let seq = seq.unwrap();
                let mut next_seq = sequences[i - 1];
                let seq = match (seq.seq_type, next_seq.seq_type) {
                    (SequenceType::Whitespace | SequenceType::Other, SequenceType::Word)
                    | (SequenceType::Other, SequenceType::Whitespace) => {
                        next_seq.end += 1;
                        next_seq
                    }
                    _ => seq,
                };
                Some(seq)
            }
            BoundaryPriority::Right if i == sequences.len() - 1 => seq,
            BoundaryPriority::Right => {
                let seq = seq.unwrap();
                let mut next_seq = sequences[i + 1];
                let seq = match (seq.seq_type, next_seq.seq_type) {
                    (SequenceType::Whitespace | SequenceType::Other, SequenceType::Word)
                    | (SequenceType::Other, SequenceType::Whitespace) => {
                        next_seq.start -= 1;
                        next_seq
                    }
                    _ => seq,
                };
                Some(seq)
            }
            BoundaryPriority::None => unreachable!(),
        },
        None => seq,
    }
}

#[derive(Copy, Clone)]
struct CharSequence {
    seq_type: SequenceType,
    start: usize,
    end: usize,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum SequenceType {
    Word,
    Whitespace,
    Other,
}

impl CharSequence {
    fn boundaries(self) -> (usize, usize) {
        (self.start, self.end)
    }
}

#[derive(Default)]
struct Sequences {
    seq: Vec<CharSequence>,
}

impl Sequences {
    pub fn new() -> Self {
        Self {
            seq: Vec::with_capacity(1),
        }
    }

    pub fn increment(&mut self, ctype: SequenceType, index: usize) {
        let last_seq_index = self.seq.len().saturating_sub(1);
        if !self.seq.is_empty() && ctype == self.seq[last_seq_index].seq_type {
            let mut sequence = &mut self.seq[last_seq_index];
            sequence.end += 1
        } else {
            let new_sequence = CharSequence {
                seq_type: ctype,
                start: index,
                end: index + 1,
            };
            self.seq.push(new_sequence)
        }
    }

    pub fn finish(self) -> Vec<CharSequence> {
        self.seq
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let actual = find_word_boundaries("", 0, BoundaryPriority::None);
        assert_eq!(actual, None);
    }

    #[test]
    fn test_origin_out_of_bounds() {
        let actual = find_word_boundaries("abc", 4, BoundaryPriority::None);
        assert_eq!(actual, None);
    }

    mod test_single_word {
        use super::*;

        #[test]
        fn test_origin_at_start() {
            ["abc", "   ", "..."].iter().for_each(|input| {
                let actual = find_word_boundaries(input, 0, BoundaryPriority::None);
                let expected = Some((0, 3));
                assert_eq!(actual, expected);
            });
        }

        #[test]
        fn test_origin_at_end() {
            ["abc", "   ", "..."].iter().for_each(|input| {
                let actual = find_word_boundaries(input, 3, BoundaryPriority::None);
                let expected = Some((0, 3));
                assert_eq!(actual, expected);
            });
        }

        #[test]
        fn test_origin_inside() {
            ["abc", "   ", "..."].iter().for_each(|input| {
                let actual = find_word_boundaries(input, 2, BoundaryPriority::None);
                let expected = Some((0, 3));
                assert_eq!(actual, expected);
            });
        }
    }

    mod test_multiple_words {
        use super::*;

        #[test]
        fn test_origin_at_start() {
            let actual = find_word_boundaries("abcdef   ...", 0, BoundaryPriority::None);
            let expected = Some((0, 6));
            assert_eq!(actual, expected);
        }

        #[test]
        fn test_origin_at_end() {
            let actual = find_word_boundaries("abcd     ,...", 13, BoundaryPriority::None);
            let expected = Some((9, 13));
            assert_eq!(actual, expected);
        }

        #[test]
        fn test_origin_inside() {
            let actual = find_word_boundaries("abcde   ,..", 2, BoundaryPriority::None);
            let expected = Some((0, 5));
            assert_eq!(actual, expected);

            let actual = find_word_boundaries("abcde   ,..", 6, BoundaryPriority::None);
            let expected = Some((5, 8));
            assert_eq!(actual, expected);

            let actual = find_word_boundaries("abcde   ,..", 10, BoundaryPriority::None);
            let expected = Some((8, 11));
            assert_eq!(actual, expected);
        }
    }

    mod test_precedence_rules {
        use super::*;

        #[test]
        fn test_word_and_whitespace() {
            let actual = find_word_boundaries("abc   def", 3, BoundaryPriority::None);
            let expected = Some((0, 3));
            assert_eq!(actual, expected);

            let actual = find_word_boundaries("abc   def", 6, BoundaryPriority::None);
            let expected = Some((6, 9));
            assert_eq!(actual, expected);
        }

        #[test]
        fn test_word_and_other() {
            let actual = find_word_boundaries("abc...def", 3, BoundaryPriority::None);
            let expected = Some((0, 3));
            assert_eq!(actual, expected);

            let actual = find_word_boundaries("abc...def", 6, BoundaryPriority::None);
            let expected = Some((6, 9));
            assert_eq!(actual, expected);
        }

        #[test]
        fn test_whitespace_and_other() {
            let actual = find_word_boundaries("   ...   ", 3, BoundaryPriority::None);
            let expected = Some((0, 3));
            assert_eq!(actual, expected);

            let actual = find_word_boundaries("   ...   ", 6, BoundaryPriority::None);
            let expected = Some((6, 9));
            assert_eq!(actual, expected);
        }
    }

    mod test_priority {
        use super::*;

        #[test]
        fn test_left_priority() {
            let actual = find_word_boundaries("   abc", 3, BoundaryPriority::Left);
            let expected = Some((0, 3));
            assert_eq!(actual, expected);

            let actual = find_word_boundaries("...abc", 3, BoundaryPriority::Left);
            let expected = Some((0, 3));
            assert_eq!(actual, expected);

            let actual = find_word_boundaries("...   ", 3, BoundaryPriority::Left);
            let expected = Some((0, 3));
            assert_eq!(actual, expected);
        }

        #[test]
        fn test_left_priority_skips_short_low_prio_sequences() {
            let actual = find_word_boundaries("abc ", 4, BoundaryPriority::Left);
            let expected = Some((0, 4));
            assert_eq!(actual, expected);

            let actual = find_word_boundaries("abc.", 4, BoundaryPriority::Left);
            let expected = Some((0, 4));
            assert_eq!(actual, expected);

            let actual = find_word_boundaries("   .", 4, BoundaryPriority::Left);
            let expected = Some((0, 4));
            assert_eq!(actual, expected);
        }

        #[test]
        fn test_right_priority() {
            let actual = find_word_boundaries("abc   ", 3, BoundaryPriority::Right);
            let expected = Some((3, 6));
            assert_eq!(actual, expected);

            let actual = find_word_boundaries("abc...", 3, BoundaryPriority::Right);
            let expected = Some((3, 6));
            assert_eq!(actual, expected);

            let actual = find_word_boundaries("   ...", 3, BoundaryPriority::Right);
            let expected = Some((3, 6));
            assert_eq!(actual, expected);
        }

        #[test]
        fn test_right_priority_skips_short_low_prio_sequences() {
            let actual = find_word_boundaries(" abc", 0, BoundaryPriority::Right);
            let expected = Some((0, 4));
            assert_eq!(actual, expected);

            let actual = find_word_boundaries(".abc", 0, BoundaryPriority::Right);
            let expected = Some((0, 4));
            assert_eq!(actual, expected);

            let actual = find_word_boundaries(".   ", 0, BoundaryPriority::Right);
            let expected = Some((0, 4));
            assert_eq!(actual, expected);
        }
    }
}
