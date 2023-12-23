import java.io.IOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;

public class Main {
  public static void main(String[] args) throws IOException {
    List<String> lines = Files.readAllLines(Paths.get("input.txt"), StandardCharsets.UTF_8);
    int lineCount = lines.size();
    int lineLength = lines.get(0).length();
    Pipe pipeMatrix[][] = new Pipe[lineCount][];
    int startX = -1;
    int startY = -1;
    for (int i = 0; i < lines.size(); i++) {
      int hasStartingPoint = lines.get(i).indexOf('S', 0);
      if (hasStartingPoint != -1) {
        startY = i;
        startX = hasStartingPoint;
      }
      pipeMatrix[i] = Pipe.fromString(lines.get(i));
    }
    if (startY == -1) {
      System.out.println("No starting point. wtf?");
      return;
    }
    int currentX = startX;
    int currentY = startY;
    int nextPosition;
    int steps = -1;
    while (true) {
      steps++;
      Pipe currentPipe = pipeMatrix[currentY][currentX];
      currentPipe.isVisited = true;
      Pipe pipeUp = null;
      Pipe pipeRight = null;
      Pipe pipeDown = null;
      Pipe pipeLeft = null;
      if (currentY > 0) {
        pipeUp = pipeMatrix[currentY - 1][currentX];
      }
      if (currentX < lineLength - 1) {
        pipeRight = pipeMatrix[currentY][currentX + 1];
      }
      if (currentY < lineCount - 1) {
        pipeDown = pipeMatrix[currentY + 1][currentX];
      }
      if (currentX > 0) {
        pipeLeft = pipeMatrix[currentY][currentX - 1];
      }
      nextPosition = walk(currentPipe, pipeUp, pipeRight, pipeDown, pipeLeft);
      if (nextPosition == 0b0000) {
        break;
      } else if (nextPosition == 0b1000) {
        currentY--;
      } else if (nextPosition == 0b0100) {
        currentX++;
      } else if (nextPosition == 0b0010) {
        currentY++;
      } else {
        currentX--;
      }
    }
    System.out.println(steps);
  }

  private static int walk(Pipe currentPipe, Pipe pipeUp, Pipe pipeRight, Pipe pipeDown, Pipe pipeLeft) {
    if (currentPipe.isAdjacent(pipeUp, 0b1000) && !pipeUp.isVisited) {
      return 0b1000;
    } else if (currentPipe.isAdjacent(pipeRight, 0b0100) && !pipeRight.isVisited) {
      return 0b0100;
    } else if (currentPipe.isAdjacent(pipeDown, 0b0010) && !pipeDown.isVisited) {
      return 0b0010;
    } else if (currentPipe.isAdjacent(pipeLeft, 0b0001) && !pipeLeft.isVisited) {
      return 0b0001;
    } else {
      return 0b0000;
    }
  }
}