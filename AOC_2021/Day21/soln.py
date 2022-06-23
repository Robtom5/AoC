DEBUG = True


def deterministic_dice():
    i = 1
    while True:
        yield i
        i += 1
        if i > 100:
            i = 1


class Player:
    def __init__(self, start_pos, name, target_score):
        self.name = name
        self.pos = start_pos
        self.score = 0
        self.target_score = target_score

    def advance(self, dice_roll):
        self.pos += dice_roll
        while self.pos > 10:
            self.pos -= 10

        self.score += self.pos
        return self.score >= self.target_score


def read_src():
    src_file = "example.txt" if DEBUG else "src.txt"
    with open(src_file, "r") as fh:
        content = fh.read().splitlines()

    players = (int(content[0][-1]), int(content[1][-1]))

    return players


def task_1():
    players = read_src()
    die = deterministic_dice()

    player1 = Player(int(players[0]), "p1", 1000)
    player2 = Player(int(players[1]), "p2", 1000)

    players = (player1, player2)
    roll = 0
    roll_max = 3000
    while roll <= roll_max:
        active_player, inactive_player = players
        roll += 3
        r1 = next(die)
        r2 = next(die)
        r3 = next(die)
        next_roll = r1 + r2 + r3
        finished = active_player.advance(next_roll)
        # print(f"{active_player.name}@{active_player.pos:>2}:{active_player.score}")
        if finished:
            # someone won
            break
        players = inactive_player, active_player
        streak = 0
    else:
        print("Nobody won")

    winner, loser = players

    winning_score = loser.score * roll
    print(f"task 1: {winning_score}")


def task_2():
    players = read_src()
    die = deterministic_dice()

    player1 = Player(int(players[0]), "p1", 21)
    player2 = Player(int(players[1]), "p2", 21)

    dirac = [1, 2, 3]
    occurences = {}
    for a in dirac:
        for b in dirac:
            for c in dirac:
                tot = a + b + c
                occurences[tot] = occurences.get(tot, 0) + 1

    print(occurences)
    # apply occurences to determine scores etc

    # winner, loser = players

    # winning_score = loser.score * roll
    print(f"task 2: {1}")


if __name__ == "__main__":
    # DEBUG = False
    task_1()
    task_2()
