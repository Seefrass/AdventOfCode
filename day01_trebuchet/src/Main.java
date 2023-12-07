import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.stream.Stream;
public class Main {
    private static final String[] numbers = {"zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};
    private static final String[] srebmun = {"orez", "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin"};

    private static int calculateIntOfLine1(String line) {
        char first = 0;
        char last = 0;
        char[] chars = line.toCharArray();

        for (char c : chars) {
            if (c >= '0' && c <= '9') {
                last = c;
                if (first == 0) {
                    first = c;
                }
            }
        }

        if (first == 0) {
            return 0;
        } else {
            String result = String.valueOf(first) + last;
            return Integer.parseInt(result);
        }
    }

    private static int calculateIntOfLine2(String line) {
        char first = 0;
        char last = 0;

        // s t r i n g -> länge: 6 character
        // 0 1 2 3 4 5

        first:
        for (int i = 0; i < line.length(); i++) {
            char c = line.charAt(i);
            if (c >= '0' && c <= '9') {
                first = c;
                break;
            } else {
                for (int j = 0; j < numbers.length; j++) {
                    if (line.startsWith(numbers[j], i)) {
                        first = (char) ('0' + j);
                        break first;
                    }
                }
            }
        }

        //reverse String for last digit
        line = new StringBuilder(line).reverse().toString();

        last:
        for (int i = 0; i < line.length(); i++) {
            char c = line.charAt(i);
            if (c >= '0' && c <= '9') {
                last = c;
                break;
            } else {
                for (int j = 0; j < srebmun.length; j++) {
                    if (line.startsWith(srebmun[j], i)) {
                        last = (char) ('0' + j);
                        break last;
                    }
                }
            }
        }

        if (first == 0 || last == 0) {
            return 0;
        } else {
            String result = String.valueOf(first) + last;
            return Integer.parseInt(result);
        }
    }

    public static void main(String[] args) {
        Path path = Path.of("input.txt");
        try (Stream<String> input = Files.lines(path); Stream<String> input2 = Files.lines(path)) {
            //int result1 = input.map(Main::calculateIntOfLine).reduce(0, (Integer::sum));
            int result1 = input.mapToInt(Main::calculateIntOfLine1).sum();
            int result2 = input2.mapToInt(Main::calculateIntOfLine2).sum();

            System.out.println("Sum of calibration values: " + result1);
            System.out.println("Sum of calibration values also considering spelled out letters: " + result2);
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}