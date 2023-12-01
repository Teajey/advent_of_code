use common::*;

type Point = (u32, u32);

#[derive(PartialEq, Debug)]
struct AxisSpan {
    length: u32,
    /// `false` means vertical
    horizontal: bool,
}

fn extrude_collinear_points(((ax, ay), (bx, by)): (Point, Point)) -> Result<(Point, AxisSpan)> {
    let dx = ax.abs_diff(bx);
    let dy = ay.abs_diff(by);

    match (dx, dy) {
        (0, 0) => Err(e!("Points are not collinear (they are the same)")),
        (0, dy) => {
            let axis_span = AxisSpan {
                length: dy,
                horizontal: false,
            };
            let point = if ay < by { (ax, ay) } else { (bx, by) };
            Ok((point, axis_span))
        }
        (dx, 0) => {
            let axis_span = AxisSpan {
                length: dx,
                horizontal: true,
            };
            let point = if ax < bx { (ax, ay) } else { (bx, by) };
            Ok((point, axis_span))
        }
        _ => Err(e!("Points are not collinear")),
    }
}

fn main() -> Result<()> {
    let input = get_input()?;

    let paths = input
        .split('\n')
        .map(|line| {
            let path = line
                .split(" -> ")
                .map(|coord_string| {
                    let [x, y] = &coord_string.split(',').collect::<Vec<_>>()[..] else {
                        return Err(e!("Expected a pair of coordinates"));
                    };

                    let x = x
                        .parse::<i32>()
                        .map_err(|err| e!("Couldn't parse {} as u32: {}", x, err))?;
                    let y = y
                        .parse::<i32>()
                        .map_err(|err| e!("Couldn't parse {} as u32: {}", y, err))?;

                    Ok((x, y))
                })
                .collect::<Result<Vec<_>>>()?;

            Ok(path)
        })
        .collect::<Result<Vec<_>>>()?;

    println!("{:#?}", paths);

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::AxisSpan;

    use super::Result;

    #[test]
    fn extrude_collinear_points() -> Result<()> {
        let result = super::extrude_collinear_points(((12, 13), (12, 12)))?;

        assert_eq!(
            (
                (12, 12),
                AxisSpan {
                    length: 1,
                    horizontal: false
                }
            ),
            result
        );

        let result = super::extrude_collinear_points(((1, 13), (20, 13)))?;

        assert_eq!(
            (
                (1, 13),
                AxisSpan {
                    length: 19,
                    horizontal: true
                }
            ),
            result
        );

        let result = super::extrude_collinear_points(((1, 1), (1, 1)));

        assert!(result.is_err());

        Ok(())
    }
}
