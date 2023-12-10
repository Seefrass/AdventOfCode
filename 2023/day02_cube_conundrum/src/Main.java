import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.stream.Stream;

public class Main {

    public static final int MAX_RED = 12;
    public static final int MAX_GREEN = 13;
    public static final int MAX_BLUE = 14;

    private static class Game {
        public int id;
        int red = 0;
        int green = 0;
        int blue = 0;

        public Game (int id) {
            this.id = id;
        }

        public Game (int id, int red, int green, int blue) {
            this.id = id;
            this.red = red;
            this.green = green;
            this.blue = blue;
        }

        public boolean isValid() {
            return red <= MAX_RED && green <= MAX_GREEN && blue <= MAX_BLUE;
        }
    }

    private static Game parseLine(String line) {
        String[] gameString = line.split(":");
        int id = Integer.parseInt(gameString[0], 5, gameString[0].length(), 10);
        String[] values = gameString[1].trim().replace(",", "").replace(";", "").split(" ");

        int red = 0;
        int green = 0;
        int blue = 0;

        for (int i = 1; i < values.length; i += 2) {
            switch (values[i]) {
                case "red" -> red = Math.max(red, Integer.parseInt(values[i - 1]));
                case "green" -> green = Math.max(green, Integer.parseInt(values[i - 1]));
                case "blue" -> blue = Math.max(blue, Integer.parseInt(values[i - 1]));
            }
        }

        return new Game(id, red, green, blue);
    }

    public static void main(String[] args) {
        Path path = Path.of("input.txt");
        try (Stream<String> input = Files.lines(path)) {
            var result = input.map(Main::parseLine).filter(Game::isValid).mapToInt(g -> g.id).sum();
            System.out.println(result);
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}