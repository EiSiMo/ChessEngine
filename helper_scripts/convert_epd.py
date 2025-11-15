# converts the stockfish test suite epd file to a csv file containing just the fen and the best move


import sys

INPUT_FILE_PATH = "STS1-STS15_LAN_v6.epd"
OUTPUT_FILE_PATH = "../src/bin/stockfish_testsuite.csv"

def parse_line(line: str) -> str | None:
    try:
        parts = line.split(';')
        if not parts:
            return None

        main_part = parts[0]
        other_parts = parts[1:]

        bm_index = main_part.find(" bm ")
        if bm_index == -1:
            return None

        fen = main_part[:bm_index].strip()
        print(f"fen: '{fen}'")
        fen += " 0 1"

        lan_move = None
        for part in other_parts:
            part = part.strip()
            if part.startswith('c9 "'):
                content_start = len('c9 "')
                content_end = part.rfind('"')

                if content_end <= content_start:
                    return None

                content = part[content_start:content_end].strip()
                if not content:
                    return None

                lan_move = content.split()[0]
                break

        if lan_move is None:
            return None

        return f"{fen},{lan_move}"

    except Exception:
        return None

def convert_file(input_path: str, output_path: str):
    try:
        with open(input_path, 'r', encoding='utf-8') as infile, \
                open(output_path, 'w', encoding='utf-8') as outfile:

            for line in infile:
                line_content = line.strip()

                if not line_content:
                    continue

                output_line = parse_line(line_content)

                if output_line:
                    outfile.write(output_line + '\n')

    except FileNotFoundError:
        print(f"Error: Input file '{input_path}' not found.", file=sys.stderr)
        sys.exit(1)
    except IOError as e:
        print(f"Error reading/writing file: {e}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"An unexpected error occurred: {e}", file=sys.stderr)
        sys.exit(1)

if __name__ == "__main__":
    convert_file(INPUT_FILE_PATH, OUTPUT_FILE_PATH)