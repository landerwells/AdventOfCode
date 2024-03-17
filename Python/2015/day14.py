reindeer_info = {
    'Dancer': {'speed': 27, 'fly_time': 5, 'rest_time': 132},
    'Cupid': {'speed': 22, 'fly_time': 2, 'rest_time': 41},
    'Rudolph': {'speed': 11, 'fly_time': 5, 'rest_time': 48},
    'Donner': {'speed': 28, 'fly_time': 5, 'rest_time': 134},
    'Dasher': {'speed': 4, 'fly_time': 16, 'rest_time': 55},
    'Blitzen': {'speed': 14, 'fly_time': 3, 'rest_time': 38},
    'Prancer': {'speed': 3, 'fly_time': 21, 'rest_time': 40},
    'Comet': {'speed': 18, 'fly_time': 6, 'rest_time': 103},
    'Vixen': {'speed': 18, 'fly_time': 5, 'rest_time': 84},
}

# Initialize points for each reindeer
points = {name: 0 for name in reindeer_info}

def simulate_race(total_time):
    distances = {name: 0 for name in reindeer_info}
    for second in range(1, total_time + 1):
        for name, reindeer in reindeer_info.items():
            cycle_time = reindeer['fly_time'] + reindeer['rest_time']
            if second % cycle_time <= reindeer['fly_time'] and second % cycle_time != 0:
                distances[name] += reindeer['speed']
        lead_distance = max(distances.values())
        for name, distance in distances.items():
            if distance == lead_distance:
                points[name] += 1
    max_distance = max(distances.values())
    max_points = max(points.values())
    return max_distance, max_points

total_seconds = 2503
max_distance, max_points = simulate_race(total_seconds)

print(f"Maximum distance traveled by any reindeer is {max_distance} km.")
print(f"Maximum points earned by any reindeer is {max_points}.")
