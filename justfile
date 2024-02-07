# Use `just work day-1 part1` to work on the specific binary for a specific day's problems
work day part:
    cargo watch -w {{day}} -x "check -p {{day}}" -s "just test {{day}} {{part}}" -s "just lint {{day}}" -s "just bench {{day}} {{part}}" 
lint day:
    cargo clippy -p {{day}} -- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used -W clippy::expect_used
test day part:
    cargo nextest run -p {{day}} {{part}}
run day part:
    cargo run --package {{day}} --bin {{part}}
bench-all:
    cargo bench -q > benchmarks.txt
bench day part:
    cargo bench --bench {{day}}-bench {{part}} >> {{day}}.bench.txt
bench-criterion day part:
    cargo bench --bench {{day}}-bench-criterion {{part}}
# create the directory for a new day's puzzle and fetch the input eg just create 4  for day 4 bootstrap
create day_num:
    cargo generate --path ./daily-template --name day-{{day_num}}
    just download {{day_num}}

# In order to use the aoc-cli you need to find your SESSION cookie inorder to interact with AoC website
# for your profile
# To obtain your session cookie, login to the Advent of Code website and inspect the session 
# value of the cookie that gets stored in your browser
# eg. 
# 1) Go to https://adventofcode.com/2022/day/1/input
# 2) right-click -> inspect -> click the "Application" tab.
# 3) Refresh
# 5) Click https://adventofcode.com under "Cookies"
# 6) Grab the value for session.
# The session cookie (a long hex string) must be provided in a single line (no line breaks) in one of the following ways (listed in order of precedence):
# - In a file called .adventofcode.session (note the dot) in your home directory (/home/alice on Linux, C:\Users\Alice on Windows, /Users/Alice on macOS).
# - In a file specified via the --session-file command line option.
# - In an ADVENT_OF_CODE_SESSION environment variable.
# - In a file called adventofcode.session (no dot) in your user's config directory (/home/alice/.config on Linux, C:\Users\Alice\AppData\Roaming on Windows, /Users/Alice/Library/Application Support on macOS).
download day_num:
    aoc download --year 2023 --day {{day_num}} --overwrite --input-file {{justfile_directory()}}/day-{{day_num}}/input1.txt --puzzle-file {{justfile_directory()}}/day-{{day_num}}/puzzle.md
    aoc download --year 2023 --day {{day_num}} --overwrite --input-only --input-file {{justfile_directory()}}/day-{{day_num}}/input2.txt

submit day_num part_num answer:
    aoc submit --year 2023 --day {{day_num}} {{part_num}} {{answer}}
