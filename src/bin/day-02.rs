use std::io;

enum ReportStatus {
    Safe,
    Unsafe,
}

enum LevelProgress {
    Increasing,
    Decreasing,
}

fn process_report(levels: &[u32]) -> ReportStatus {
    let progression = if levels[0] > levels[1] {
        LevelProgress::Decreasing
    } else {
        LevelProgress::Increasing
    };

    for w in levels.windows(2) {
        match progression {
            LevelProgress::Increasing => {
                if w[0] > w[1] {
                    return ReportStatus::Unsafe;
                }
            },
            LevelProgress::Decreasing => {
                if w[0] < w[1] {
                    return ReportStatus::Unsafe;
                }
            },
        }
        if w[0].abs_diff(w[1]) < 1 || w[0].abs_diff(w[1]) > 3 {
            return ReportStatus::Unsafe;
        }
    }

    ReportStatus::Safe
}

fn try_fix_unsafe(levels: &[u32]) -> ReportStatus {
    // Try to make the report safe by removing a level from the list, for all the levels in the list.
    for i in 0..levels.len() {
        let mut possible: Vec<_> = Vec::from(levels);
        possible.remove(i);
        match process_report(&possible) {
            ReportStatus::Safe => return ReportStatus::Safe,
            _ => continue,
        }
    }

    ReportStatus::Unsafe
}

fn main() -> anyhow::Result<()> {
    let input = io::read_to_string(io::stdin())?;

    let mut nsafe = 0;
    for line in input.lines() {
        let levels: Vec<_> = line
            .split_whitespace()
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()?;

        match process_report(&levels) {
            ReportStatus::Safe => nsafe += 1,
            ReportStatus::Unsafe => match try_fix_unsafe(&levels) {
                ReportStatus::Safe => nsafe += 1,
                ReportStatus::Unsafe => {},
            },
        }
    }

    println!("Number of safe reports: {nsafe}");

    Ok(())
}
