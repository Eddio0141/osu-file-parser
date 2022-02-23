#[cfg(test)]
mod tests {
    use crate::osu_file::general::General;

    #[test]
    fn general_match() {
        let input = "
AudioFilename: test.mp3
AudioLeadIn: asdf.mp3
AudioHash: no.mp3
PreviewTime: 5
Countdown: 3
SampleSet: Soft
StackLeniency: 0.9
Mode: 1
LetterboxInBreaks: 1
StoryFireInFront: 0
UseSkinSprites: 1
AlwaysShowPlayfield: 0
OverlayPosition: Above
SkinPreference: myskin
EpilepsyWarning: 1
CountdownOffset: 120
SpecialStyle: 1
WidescreenStoryboard: 1
SamplesMatchPlaybackRate: 1
        ";

        let output = input.parse::<General>().unwrap().to_string();

        assert_eq!(input, output);
    }
}

pub mod osu_file;
