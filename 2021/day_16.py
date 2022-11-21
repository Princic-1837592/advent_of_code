class Packet:
    def __init__(self, bits: str, index: int):
        self.length = 0
        self.packets = None
        self.value = None
        self.index = index
        self.version = int(bits[index:index + 3], 2)
        index += 3
        self.type_id = int(bits[index:index + 3], 2)
        index += 3
        if self.type_id == 4:
            self.parse_type_4(bits, index)
        else:
            self.parse_other(bits, index)
        index += self.length

    def parse_type_4(self, bits: str, index: int):
        value = ''
        while True:
            value += bits[index + 1:index + 5]
            index += 5
            if bits[index - 5] == '0':
                break
        self.value = int(value, 2)
        self.length = index - self.index

    def parse_other(self, bits: str, index: int):
        if bits[index] == '0':
            self.parse_label_0(bits, index + 1)
        else:
            self.parse_label_1(bits, index + 1)

    def parse_label_0(self, bits: str, index: int):
        self.packets = []
        num_bits = int(bits[index:index + 15], 2)
        index += 15
        while index < self.index + 7 + 15 + num_bits:
            self.packets.append(Packet(bits, index))
            index += self.packets[-1].length
        self.length = index - self.index

    def parse_label_1(self, bits: str, index: int):
        num_packets = int(bits[index:index + 11], 2)
        index += 11
        self.packets = []
        for i in range(num_packets):
            self.packets.append(Packet(bits, index))
            index += self.packets[i].length
        self.length = index - self.index

    def get_total_version(self):
        version = self.version
        if self.packets:
            for packet in self.packets:
                version += packet.get_total_version()
        return version

    def evaluate(self):
        if self.type_id == 0:
            return sum(packet.evaluate() for packet in self.packets)
        if self.type_id == 1:
            product = 1
            for packet in self.packets:
                product *= packet.evaluate()
            return product
        if self.type_id == 2:
            return min(packet.evaluate() for packet in self.packets)
        if self.type_id == 3:
            return max(packet.evaluate() for packet in self.packets)
        if self.type_id == 4:
            return self.value
        if self.type_id == 5:
            return int(self.packets[0].evaluate() > self.packets[1].evaluate())
        if self.type_id == 6:
            return int(self.packets[0].evaluate() < self.packets[1].evaluate())
        if self.type_id == 7:
            return int(self.packets[0].evaluate() == self.packets[1].evaluate())

    def __str__(self):
        result = f'V: {self.version}, T: {self.type_id}, {{}}'
        if self.type_id == 4:
            result = result.format(f'{self.value}')
        else:
            result = result.format(f'[{", ".join(list(map(str, self.packets)))}]')
        return f'{{{result}}}'


def part1(data: str):
    bits = str.format('{:b}', int(data, 16))
    bits = '0' * (4 * len(data) - len(bits)) + bits
    packet = Packet(bits, 0)
    return packet.get_total_version()


def part2(data: str):
    bits = str.format('{:b}', int(data, 16))
    bits = '0' * (4 * len(data) - len(bits)) + bits
    packet = Packet(bits, 0)
    return packet.evaluate()


if __name__ == '__main__':
    test = True
    test = False
    test_input = '''A0016C880162017C3686B18A3D4780'''
    if test:
        puzzle_input = test_input
    else:
        with open('day_16_input.txt', 'r') as input_file:
            puzzle_input = input_file.read().strip()
    print(part1(puzzle_input))
    print(part2(puzzle_input))
