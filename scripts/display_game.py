import chess
import chess.svg

boards = []
while True:
    line = input()
    if line:
        if not line.startswith("Finding the best move took"):
            boards.append(line)
    else:
        break

svgs = []
for fen in boards:
    b = chess.Board(fen)
    svgs.append(chess.svg.board(b, size=350))

l = "\n<br>\n".join([x for x in svgs])


with open("out.html", "w") as file:
    file.write(l)
