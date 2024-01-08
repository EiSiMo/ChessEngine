import chess
import chess.engine
import subprocess
import multiprocessing
import statistics
import time
import sys


class Benchmarker:
    def __init__(self):
        subprocess.run("cargo build --release")

        self.amount_of_processes = 8
        self.work = multiprocessing.Queue()
        manager = multiprocessing.Manager()
        self.total_moves = 0
        self.results_dict = manager.dict()
        self.results_dict["correct_moves"] = 0

    def benchmark(self):
        print(f"[*] Benchmarking engine")

        with open("stockfish_moves.txt", "r", encoding="utf8") as file:
            data = file.read()
            lines = data.split("\n")

        self.total_moves = len(lines)
        for line in lines:
            board_fen = line.split(" | ")[1]
            best_move = line.split(" | ")[0]

            self.work.put((board_fen, best_move))

        processes = []
        for _ in range(self.amount_of_processes):
            process = multiprocessing.Process(target=self.threaded_benchmark)
            process.start()
            processes.append(process)

        while True:
            current = self.work.qsize()
            rest = round(current / (len(lines) / 100))
            percentage = 100 - rest
            sys.stdout.write('\r')
            # the exact output you're looking for:
            sys.stdout.write(f"[*] [{'='*round(percentage/5)}{' '*round(rest/5)}] {percentage}%")
            sys.stdout.flush()
            time.sleep(0.25)

            close = True
            for process in processes:
                if process.is_alive():
                    close = False
            if close:
                sys.stdout.write('\r')
                sys.stdout.write(f"[*] Benchmark finished")
                sys.stdout.flush()
                print()
                break

    def threaded_benchmark(self):
        engine = chess.engine.SimpleEngine.popen_uci(r"../../target/release/ChessEngine.exe")
        correct_moves = 0
        while True:
            if self.work.empty():
                break
            work = self.work.get()
            if work is not None:
                fen, best_move = work

                board = chess.Board(fen)

                result = engine.play(board, chess.engine.Limit(time=1000))

                if str(result.move) == str(best_move):
                    self.results_dict["correct_moves"] += 1
        self.results_dict["correct_moves"] += correct_moves
        engine.quit()

    def show_results(self):
        print("[*] Results")
        print(f"\taccuracy:\t{self.results_dict["correct_moves"]}/{self.total_moves} ({round(self.results_dict["correct_moves"]/(self.total_moves/100))}%)")


if __name__ == "__main__":
    b1 = Benchmarker()
    b1.benchmark()
    b1.show_results()
