import tkinter as tk

class ChessBitboardApp:
    def __init__(self, root):
        self.root = root
        self.root.title("Chess Bitboard Generator")

        # The bitboard, stored as a 64-bit integer
        self.bitboard = 0

        # A dictionary to keep track of squares and their marked state
        # Key: (row, col), Value: {
        #   'widget': tk.Frame, 'label_alg': tk.Label, 'label_idx': tk.Label,
        #   'marked': bool, 'original_color': str
        # }
        self.squares = {}

        # --- Create the GUI ---

        # Frame for the chessboard
        board_frame = tk.Frame(root)
        board_frame.pack()

        # Create the 8x8 grid of squares
        # We loop from row 7 (rank 8) down to 0 (rank 1) for visual layout
        for r in range(7, -1, -1):  # 7, 6, 5, ... 0
            for c in range(8):      # 0, 1, 2, ... 7

                # Determine the square's original color
                is_light_square = (r + c) % 2 == 1
                original_color = "#F0D9B5" if is_light_square else "#B58863"

                # Create the square as a Frame
                square = tk.Frame(
                    board_frame,
                    width=50,
                    height=50,
                    bg=original_color,
                    relief="sunken",
                    borderwidth=1
                )
                # Make frame *not* resize to labels
                square.pack_propagate(False) # Use pack_propagate since we use place/pack inside
                square.grid(row=7 - r, column=c)

                # --- Add labels to the square (Feature 2) ---
                algebraic_not = f"{'abcdefgh'[c]}{r + 1}"
                bit_index = r * 8 + c

                label_alg = tk.Label(square, text=algebraic_not, bg=original_color, font=("Arial", 8, "bold"))
                label_alg.place(x=2, y=1) # Use place to position label

                label_idx = tk.Label(square, text=f"{bit_index}", bg=original_color, font=("Arial", 8))
                label_idx.place(relx=1.0, rely=1.0, anchor='se', x=-2, y=-1) # Use place for bottom-right

                # Bind the click event to all parts of the square
                click_lambda = lambda event, row=r, col=c: self.on_square_click(event, row, col)
                square.bind("<Button-1>", click_lambda)
                label_alg.bind("<Button-1>", click_lambda)
                label_idx.bind("<Button-1>", click_lambda)

                # Store the square's info
                self.squares[(r, c)] = {
                    'widget': square,
                    'label_alg': label_alg,
                    'label_idx': label_idx,
                    'marked': False,
                    'original_color': original_color
                }

        # Frame for the bitboard display
        info_frame = tk.Frame(root, pady=10)
        info_frame.pack()

        # --- Make display labels copyable (Feature 1) ---
        self.binary_var = tk.StringVar()
        self.int_var = tk.StringVar()

        tk.Label(info_frame, text="Binary:").pack()
        self.binary_label = tk.Entry(
            info_frame,
            textvariable=self.binary_var,
            state="readonly",
            font=("Courier", 10),
            width=77 # 64 chars + 15 underscores + 'b'
        )
        self.binary_label.pack()

        tk.Label(info_frame, text="Integer:").pack()
        self.int_label = tk.Entry(
            info_frame,
            textvariable=self.int_var,
            state="readonly",
            font=("Courier", 12, "bold"),
            width=22
        )
        self.int_label.pack()

        # --- Add Entry for pasting bitboard (Feature 3) ---
        input_frame = tk.Frame(root, pady=5)
        input_frame.pack()

        tk.Label(input_frame, text="Paste Bitboard (int) and Press Enter:").pack(side=tk.LEFT)
        self.paste_entry = tk.Entry(input_frame, font=("Courier", 12), width=22)
        self.paste_entry.pack(side=tk.LEFT, padx=5)
        self.paste_entry.bind("<Return>", self.on_paste_bitboard)

        # Initialize display
        self.update_display()

    def on_square_click(self, event, row, col):
        """Handles the click event for a square."""
        square_info = self.squares[(row, col)]

        # Toggle the marked state
        square_info['marked'] = not square_info['marked']

        self.update_square_visuals(row, col)

        # Recalculate the bitboard and update the display
        self.recalculate_bitboard()
        self.update_display()

    def update_square_visuals(self, row, col):
        """Updates a single square's color based on its 'marked' state."""
        square_info = self.squares[(row, col)]

        is_marked = square_info['marked']
        new_color = "#50C878" if is_marked else square_info['original_color']
        label_fg_color = "white" if is_marked else "black" # Make text white on green

        square_info['widget'].config(bg=new_color)
        square_info['label_alg'].config(bg=new_color, fg=label_fg_color)
        square_info['label_idx'].config(bg=new_color, fg=label_fg_color)

    def recalculate_bitboard(self):
        """Recalculates the 64-bit integer from the marked squares."""
        self.bitboard = 0

        for r in range(8):
            for c in range(8):
                if self.squares[(r, c)]['marked']:
                    bit_index = r * 8 + c
                    self.bitboard |= (1 << bit_index)

    def update_display(self):
        """Updates the binary and integer labels."""

        # Update the integer label
        self.int_var.set(f"{self.bitboard}")

        # Update the binary label
        binary_string = f"{self.bitboard:064b}"
        formatted_binary = "b" + "_".join(binary_string[i:i+8] for i in range(0, 64, 4))
        self.binary_var.set(f"{formatted_binary}")

    def on_paste_bitboard(self, event):
        """Handles the 'Enter' key press in the paste entry box."""
        try:
            # Get text from entry and convert to integer
            new_bitboard = int(self.paste_entry.get())
            if 0 <= new_bitboard <= (1 << 64) - 1:
                self.bitboard = new_bitboard
                # Update the board visuals from the new bitboard
                self.update_board_from_bitboard()
                # Update the display labels
                self.update_display()
            else:
                # Handle out-of-range numbers
                self.paste_entry.delete(0, tk.END)
                self.paste_entry.insert(0, "Out of 64-bit range")

        except ValueError:
            # Handle non-integer input
            self.paste_entry.delete(0, tk.END)
            self.paste_entry.insert(0, "Invalid Integer")

    def update_board_from_bitboard(self):
        """Updates the visual state of all squares based on self.bitboard."""
        for r in range(8):
            for c in range(8):
                bit_index = r * 8 + c
                square_info = self.squares[(r, c)]

                # Check if the bit at bit_index is set
                if (self.bitboard >> bit_index) & 1:
                    square_info['marked'] = True
                else:
                    square_info['marked'] = False

                # Update the square's color
                self.update_square_visuals(r, c)

if __name__ == "__main__":
    root = tk.Tk()
    app = ChessBitboardApp(root)
    root.mainloop()