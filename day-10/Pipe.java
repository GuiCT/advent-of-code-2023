public class Pipe {
  public int directions;
  public boolean isVisited;

  public Pipe(char c) {
    this.directions = switch(c) {
      case '|' -> 0b1010;
      case '-' -> 0b0101;
      case 'L' -> 0b1100;
      case 'J' -> 0b1001;
      case '7' -> 0b0011;
      case 'F' -> 0b0110;
      default -> 0;
    };
    this.isVisited = false;
  }

  public static Pipe[] fromString(String string) {
    Pipe pipes[] = new Pipe[string.length()];
    char charArray[] = string.toCharArray();
    for (int i = 0; i < charArray.length; i++) {
      pipes[i] = new Pipe(charArray[i]);
    }
    return pipes;
  }

  public boolean isAdjacent(Pipe otherPipe, int position) {
    if (otherPipe == null) {
      return false;
    }
    int baseMask = this.directions & position;
    if (baseMask == 0b0000) {
      return false;
    }
    if (position >= 0b0100) {
      return (((baseMask >> 2) & otherPipe.directions) > 0b0000);
    } else {
      return (((baseMask << 2) & otherPipe.directions) > 0b0000);
    }
  }
}
