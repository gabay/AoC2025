import itertools

import matplotlib.pyplot as plt
import shapely
import shapely.plotting

points = [list(map(int, line.split(","))) for line in open("9").read().splitlines()]
polygon = shapely.Polygon(points)
shapely.plotting.plot_polygon(polygon)


def area(p1p2):
    p1, p2 = p1p2
    return abs(p1[0] - p2[0]) * abs(p1[1] - p2[1])


(x1, y1), (x2, y2) = max(itertools.combinations(points, 2), key=area)
biggest_rect = shapely.box(min(x1, x2), min(y1, y2), max(x1, x2), max(y1, y2))
shapely.plotting.plot_polygon(biggest_rect)

(x3, y3), (x4, y4) = [4318.0, 65238.0], [94987.0, 50332.0]
biggest_rect2 = shapely.box(min(x3, x4), min(y3, y4), max(x3, x4), max(y3, y4))
shapely.plotting.plot_polygon(biggest_rect2)


plt.show()
