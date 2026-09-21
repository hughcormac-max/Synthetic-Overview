import json
import math

with open('assets/data/solar_system.json', 'r', encoding='utf-8') as f:
    data = json.load(f)

# Inclination and Longitude of Ascending Node (in degrees)
orbital_params = {
    "Sun": (0.0, 0.0),
    "Mercury": (7.00, 48.331),
    "Venus": (3.39, 76.680),
    "Earth": (0.0, 0.0),
    "Luna": (5.14, 125.08),
    "Mars": (1.85, 49.578),
    "Jupiter": (1.30, 100.556),
    "Io": (0.04, 43.9),
    "Europa": (0.47, 219.1),
    "Ganymede": (0.18, 63.5),
    "Callisto": (0.19, 298.8),
    "Saturn": (2.49, 113.715),
    "Titan": (0.33, 24.6),
    "Enceladus": (0.0, 0.0),
    "Uranus": (0.77, 74.229),
    "Titania": (0.34, 165.9),
    "Oberon": (0.058, 274.7),
    "Neptune": (1.77, 131.721),
    "Triton": (156.8, 109.6)
}

for body in data:
    name = body["name"]
    inc_deg, lan_deg = orbital_params.get(name, (0.0, 0.0))
    inc_rad = math.radians(inc_deg)
    lan_rad = math.radians(lan_deg)
    
    long_peri_rad = body.pop("longitude_of_periapsis_rad")
    arg_peri_rad = (long_peri_rad - lan_rad) % (2 * math.pi)
    
    # insert the new properties where longitude_of_periapsis_rad was
    # to keep JSON ordered reasonably
    
    body["inclination_rad"] = round(inc_rad, 5)
    body["longitude_of_ascending_node_rad"] = round(lan_rad, 5)
    body["argument_of_periapsis_rad"] = round(arg_peri_rad, 5)

# rewrite using json.dump
with open('assets/data/solar_system.json', 'w', encoding='utf-8') as f:
    json.dump(data, f, indent=2)

