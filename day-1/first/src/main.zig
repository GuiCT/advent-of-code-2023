const std = @import("std");
const IntPair = std.meta.Tuple(&.{ u8, u8 });

fn isDigit(c: u8) bool {
    return c >= '0' and c <= '9';
}

fn get_first_and_last_digits(line: []u8) IntPair {
    var first_counter: usize = 0;
    var last_counter: usize = line.len - 1;
    var found_first: bool = false;
    var found_last: bool = false;
    while (!found_first or !found_last) {
        if (!found_first) {
            if (isDigit(line[first_counter])) {
                found_first = true;
            } else {
                first_counter += 1;
            }
        }

        if (!found_last) {
            if (isDigit(line[last_counter])) {
                found_last = true;
            } else {
                last_counter -= 1;
            }
        }
    }
    return .{ line[first_counter], line[last_counter] };
}

pub fn main() !void {
    const stdin = std.io.getStdIn().reader();
    const stdout = std.io.getStdOut().writer();
    const out_buffered_writer = std.io.bufferedWriter(stdout);
    _ = out_buffered_writer;
    var buffer: [1048576]u8 = undefined;
    var number_input_buffer: [2]u8 = undefined;
    var sum: i64 = 0;

    while (try stdin.readUntilDelimiterOrEof(&buffer, '\n')) |l| {
        if (l.len < 2) {
            break;
        }
        var pair_line = get_first_and_last_digits(l);
        number_input_buffer[0] = pair_line[0];
        number_input_buffer[1] = pair_line[1];
        var value = try std.fmt.parseInt(i32, &number_input_buffer, 10);
        sum += value;
    }

    try stdout.print("{d}\n", .{sum});
}
