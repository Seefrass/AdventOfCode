import java.awt.*;
import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.*;
import java.util.List;
import java.util.stream.Stream;

public class Main {
    public static boolean is_digit(char c) {
        return c >= '0'&& c <= '9';
    }

    private static int calculateSum(char[][] schematic) {
        int result = 0;
        for (int i = 0; i < schematic.length; i++) {
            for (int j = 0; j < schematic[i].length; j++) {
                if (is_digit(schematic[i][j])) {
                    int num_start = j;
                    int num_length = 0;
                    for (int k = num_start; k < schematic[i].length; k++) { //determine length of number
                        if (is_digit(schematic[i][k])) {
                            num_length++;
                        } else {
                            break;
                        }
                    }

                    StringBuilder num_str = new StringBuilder();

                    for (int k = num_start; k < num_start + num_length; k++) { //get number as String
                        num_str.append(schematic[i][k]);
                    }

                    int num = Integer.parseInt(num_str.toString());

                    number:
                    for (int k = Math.max(i - 1, 0); k <= Math.min(i + 1, schematic.length - 1); k++) {
                        for (int l = Math.max(num_start - 1, 0); l <= Math.min(num_start + num_length, schematic[i].length - 1); l++) {
                            if (schematic[k][l] != '.' && !is_digit(schematic[k][l])) {
                                result += num;
                                j += num_length;
                                break number;
                            }
                        }
                    }
                }
            }
        }
        return result;
    }

    private static int calculateGearRatios(char[][] schematic) {
        HashMap<Point, List<Integer>> map = new HashMap<>();

        for (int i = 0; i < schematic.length; i++) {
            for (int j = 0; j < schematic[i].length; j++) {
                if (is_digit(schematic[i][j])) {
                    int num_start = j;
                    int num_length = 0;
                    for (int k = num_start; k < schematic[i].length; k++) { //determine length of number
                        if (is_digit(schematic[i][k])) {
                            num_length++;
                        } else {
                            break;
                        }
                    }

                    StringBuilder num_str = new StringBuilder();

                    for (int k = num_start; k < num_start + num_length; k++) { //get number as String
                        num_str.append(schematic[i][k]);
                    }

                    int num = Integer.parseInt(num_str.toString());

                    for (int k = Math.max(i - 1, 0); k <= Math.min(i + 1, schematic.length - 1); k++) {
                        for (int l = Math.max(num_start - 1, 0); l <= Math.min(num_start + num_length, schematic[i].length - 1); l++) {
                            if (schematic[k][l] == '*') {
                                List<Integer> entries = map.getOrDefault(new Point(k, l), new ArrayList<>());
                                entries.add(num);
                                map.put(new Point(k, l), entries);
                            }
                        }
                    }
                    j += num_length;
                }
            }
        }
        return map.values().stream().filter(integers -> integers.size() == 2).mapToInt(integers -> integers.get(0) * integers.get(1)).sum();
    }

    public static void main(String[] args) {
        Path path = Path.of("input.txt");
        try (Stream<String> input = Files.lines(path)) {
            BufferedReader reader = new BufferedReader(new FileReader("input.txt"));
            int length = (int) input.count();
            char[][] schematic = new char[length][];
            for (int i = 0; i < length; i++) {
                schematic[i] = reader.readLine().toCharArray();
            }

            int result1 = calculateSum(schematic);
            int result2 = calculateGearRatios(schematic);
            System.out.println("Sum of all valid numbers in the schematic is: " + result1);
            System.out.println("Sum of all Gear Ratios is: " + result2);

        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}