import chess
import chess.engine
import subprocess
import multiprocessing
import statistics
import time
import sys


class Benchmarker:
    def __init__(self):
        print("[*] Compiling competitor engine")
        subprocess.run("cargo build --release")
        print("[*] Starting engines")
        self.amount_of_games = 100
        self.amount_of_processes = 8
        self.work = multiprocessing.Queue()
        manager = multiprocessing.Manager()
        self.results = manager.dict()
        self.results["stockfish_wins"] = 0
        self.results["competitor_wins"] = 0
        self.results["draws"] = 0
        self.results["amount_of_moves_on_competitor_losses"] = manager.list()

    def benchmark(self):
        print(f"[*] Benchmarking 'COMPETITOR' vs 'STOCKFISH 16'")
        processes = []

        for game_count in range(1, self.amount_of_games+1):
            self.work.put((game_count % 2 == 0))

        for _ in range(self.amount_of_processes):
            process = multiprocessing.Process(target=self.threaded_stockfish_vs_competitor)
            process.start()
            processes.append(process)

        while True:
            current = self.work.qsize()
            rest = round(current / (self.amount_of_games / 100))
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

    def threaded_stockfish_vs_competitor(self):
        competitor = chess.engine.SimpleEngine.popen_uci(r"../../target/release/ChessEngine.exe")
        stockfish = chess.engine.SimpleEngine.popen_uci(r"stockfish/stockfish-windows-x86-64.exe")

        while True:
            if self.work.empty():
                break
            work = self.work.get()
            if work is not None:
                stockfish_plays_withe = work

                stockfish_timelimit = 0.1
                competitor_timelimit = 1000.0
                move_count = 0
                board = chess.Board()

                if stockfish_plays_withe:
                    result = stockfish.play(board, chess.engine.Limit(time=stockfish_timelimit))
                    board.push(result.move)
                    move_count += 1

                while True:
                    result = competitor.play(board, chess.engine.Limit(time=competitor_timelimit))
                    board.push(result.move)
                    move_count += 1
                    if board.is_game_over():
                        break

                    result = stockfish.play(board, chess.engine.Limit(time=stockfish_timelimit))
                    board.push(result.move)
                    move_count += 1
                    if board.is_game_over():
                        break

                self.results["amount_of_moves_on_competitor_losses"].append(move_count)
                outcome = board.outcome()
                if outcome:
                    if outcome.winner == chess.WHITE:
                        if stockfish_plays_withe:
                            self.results["stockfish_wins"] += 1
                        else:
                            self.results["competitor_wins"] += 1
                    elif outcome.winner == chess.BLACK:
                        if stockfish_plays_withe:
                            self.results["competitor_wins"] += 1
                        else:
                            self.results["stockfish_wins"] += 1
                    else:
                        self.results["draws"] += 1
            else:
                break
        stockfish.quit()
        competitor.quit()

    def show_results(self):
        mv_amt_list = list(self.results['amount_of_moves_on_competitor_losses'])
        mv_amt_list.sort()

        print("[*] Results")
        print(f"\tgames played:\t\t    {self.amount_of_games}")
        print(f"\tstockfish wins:\t\t    {self.results['stockfish_wins']}")
        print(f"\tcompetitor wins:\t    {self.results['competitor_wins']}")
        print(f"\tdraws:\t\t\t\t    {self.results['draws']}")

        print(f"\tavg moves/game:\t\t\t{round(statistics.mean(mv_amt_list), 2)}")
        print(f"\tbest moves/game:\t    {mv_amt_list[-1]}")
        print(f"\tworst moves/game:\t    {mv_amt_list[0]}")


if __name__ == "__main__":
    b1 = Benchmarker()
    b1.benchmark()
    b1.show_results()
