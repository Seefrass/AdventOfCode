import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Arrays;
import java.util.stream.Stream;

public class Main {

    private static int evaluateCard(String s) {
        String[] cardStr = s.split(":");
        String[] dataStr = cardStr[1].trim().split("\\|");
        int[] winNums = Arrays.stream(dataStr[0].trim().split(" ")).filter(str -> !str.isBlank()).mapToInt(Integer::parseInt).toArray();
        int[] nums = Arrays.stream(dataStr[1].trim().split(" ")).filter(str -> !str.isBlank()).mapToInt(Integer::parseInt).toArray();
        int result = 0;
        for (int winNum : winNums) {
            for (int num : nums) {
                if (winNum == num) {
                    result = (result == 0 ? 1 : result * 2);
                }
            }
        }
        return result;
    }

    public static void main(String[] args) {
        Path path = Path.of("input.txt");
        try (Stream<String> input = Files.lines(path)) {
            int result = input.mapToInt(Main::evaluateCard).sum(); //48931
            System.out.println("Sum of all points is: " + result);
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}