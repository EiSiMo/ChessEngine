# this is a really scrappy script to transform a human readable chess board into a chess board, readable by this engine

field = []
print("   ABCDEFGH")
for i in range(8):
    l = list(input(f"{8-i}: "))
    if len(l) != 8:
        l.extend([" "] * (8-len(l)))
    field.extend(l)


bin_pawns = ""
bin_bishops = ""
bin_knights = ""
bin_rooks = ""
bin_queens = ""
bin_kings = ""

bin_black_pawns = ""
bin_black_bishops = ""
bin_black_knights = ""
bin_black_rooks = ""
bin_black_queens = ""
bin_black_kings = ""

for square in field:
    if square == "P":
        bin_pawns += "1"
        bin_bishops += "0"
        bin_knights += "0"
        bin_rooks += "0"
        bin_queens += "0"
        bin_kings += "0"
        bin_black_pawns += "0"
        bin_black_bishops += "0"
        bin_black_knights += "0"
        bin_black_rooks += "0"
        bin_black_queens += "0"
        bin_black_kings += "0"
    elif square == "B":
        bin_pawns += "0"
        bin_bishops += "1"
        bin_knights += "0"
        bin_rooks += "0"
        bin_queens += "0"
        bin_kings += "0"
        bin_black_pawns += "0"
        bin_black_bishops += "0"
        bin_black_knights += "0"
        bin_black_rooks += "0"
        bin_black_queens += "0"
        bin_black_kings += "0"
    elif square == "K":
        bin_pawns += "0"
        bin_bishops += "0"
        bin_knights += "1"
        bin_rooks += "0"
        bin_queens += "0"
        bin_kings += "0"
        bin_black_pawns += "0"
        bin_black_bishops += "0"
        bin_black_knights += "0"
        bin_black_rooks += "0"
        bin_black_queens += "0"
        bin_black_kings += "0"
    elif square == "R":
        bin_pawns += "0"
        bin_bishops += "0"
        bin_knights += "0"
        bin_rooks += "1"
        bin_queens += "0"
        bin_kings += "0"
        bin_black_pawns += "0"
        bin_black_bishops += "0"
        bin_black_knights += "0"
        bin_black_rooks += "0"
        bin_black_queens += "0"
        bin_black_kings += "0"
    elif square == "Q":
        bin_pawns += "0"
        bin_bishops += "0"
        bin_knights += "0"
        bin_rooks += "0"
        bin_queens += "1"
        bin_kings += "0"
        bin_black_pawns += "0"
        bin_black_bishops += "0"
        bin_black_knights += "0"
        bin_black_rooks += "0"
        bin_black_queens += "0"
        bin_black_kings += "0"
    elif square == "G":
        bin_pawns += "0"
        bin_bishops += "0"
        bin_knights += "0"
        bin_rooks += "0"
        bin_queens += "0"
        bin_kings += "1"
        bin_black_pawns += "0"
        bin_black_bishops += "0"
        bin_black_knights += "0"
        bin_black_rooks += "0"
        bin_black_queens += "0"
        bin_black_kings += "0"
    elif square == "p":
        bin_pawns += "0"
        bin_bishops += "0"
        bin_knights += "0"
        bin_rooks += "0"
        bin_queens += "0"
        bin_kings += "0"
        bin_black_pawns += "1"
        bin_black_bishops += "0"
        bin_black_knights += "0"
        bin_black_rooks += "0"
        bin_black_queens += "0"
        bin_black_kings += "0"
    elif square == "b":
        bin_pawns += "0"
        bin_bishops += "0"
        bin_knights += "0"
        bin_rooks += "0"
        bin_queens += "0"
        bin_kings += "0"
        bin_black_pawns += "0"
        bin_black_bishops += "1"
        bin_black_knights += "0"
        bin_black_rooks += "0"
        bin_black_queens += "0"
        bin_black_kings += "0"
    elif square == "k":
        bin_pawns += "0"
        bin_bishops += "0"
        bin_knights += "0"
        bin_rooks += "0"
        bin_queens += "0"
        bin_kings += "0"
        bin_black_pawns += "0"
        bin_black_bishops += "0"
        bin_black_knights += "1"
        bin_black_rooks += "0"
        bin_black_queens += "0"
        bin_black_kings += "0"
    elif square == "r":
        bin_pawns += "0"
        bin_bishops += "0"
        bin_knights += "0"
        bin_rooks += "0"
        bin_queens += "0"
        bin_kings += "0"
        bin_black_pawns += "0"
        bin_black_bishops += "0"
        bin_black_knights += "0"
        bin_black_rooks += "1"
        bin_black_queens += "0"
        bin_black_kings += "0"
    elif square == "q":
        bin_pawns += "0"
        bin_bishops += "0"
        bin_knights += "0"
        bin_rooks += "0"
        bin_queens += "0"
        bin_kings += "0"
        bin_black_pawns += "0"
        bin_black_bishops += "0"
        bin_black_knights += "0"
        bin_black_rooks += "0"
        bin_black_queens += "1"
        bin_black_kings += "0"
    elif square == "g":
        bin_pawns += "0"
        bin_bishops += "0"
        bin_knights += "0"
        bin_rooks += "0"
        bin_queens += "0"
        bin_kings += "0"
        bin_black_pawns += "0"
        bin_black_bishops += "0"
        bin_black_knights += "0"
        bin_black_rooks += "0"
        bin_black_queens += "0"
        bin_black_kings += "1"
    else:
        bin_pawns += "0"
        bin_bishops += "0"
        bin_knights += "0"
        bin_rooks += "0"
        bin_queens += "0"
        bin_kings += "0"
        bin_black_pawns += "0"
        bin_black_bishops += "0"
        bin_black_knights += "0"
        bin_black_rooks += "0"
        bin_black_queens += "0"
        bin_black_kings += "0"

print("withe pawns: ", bin_pawns, int(bin_pawns, 2))
print("withe bishops: ", bin_bishops, int(bin_bishops, 2))
print("withe knights: ", bin_knights, int(bin_knights, 2))
print("withe rooks: ", bin_rooks, int(bin_rooks, 2))
print("withe queens: ", bin_queens, int(bin_queens, 2))
print("withe kings: ", bin_kings, int(bin_kings, 2))
print("black pawns: ", bin_black_pawns, int(bin_black_pawns, 2))
print("black bishops: ", bin_black_bishops, int(bin_black_bishops, 2))
print("black knights: ", bin_black_knights, int(bin_black_knights, 2))
print("black rooks: ", bin_black_rooks, int(bin_black_rooks, 2))
print("black queens: ", bin_black_queens, int(bin_black_queens, 2))
print("black kings: ", bin_black_kings, int(bin_black_kings, 2))

print(f"""
Board {{
    withe_pawns: {int(bin_pawns, 2)},
    withe_bishops: {int(bin_bishops, 2)},
    withe_knights: {int(bin_knights, 2)},
    withe_rooks: {int(bin_rooks, 2)},
    withe_queens: {int(bin_queens, 2)},
    withe_king: {int(bin_kings, 2)},
    black_pawns: {int(bin_black_pawns, 2)},
    black_bishops: {int(bin_black_bishops, 2)},
    black_knights: {int(bin_black_knights, 2)},
    black_rooks: {int(bin_black_rooks, 2)},
    black_queens: {int(bin_black_queens, 2)},
    black_king: {int(bin_black_kings, 2)}
}}
""")
