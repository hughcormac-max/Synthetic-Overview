import re

with open('docs/ssot/SSOT-PHY-004-AstroNode-Architecture.md', encoding='utf-8') as f:
    content = f.read()

content = content.replace('2. **2D Keplerian Orbits.** The solar system simulation is strictly 3D, but all planetary orbits are coplanar. Thus, `AstroNode` positions are derived using 2D Keplerian elements relative to their parent body.', '2. **3D Keplerian Orbits.** The solar system simulation is strictly 3D, and planetary orbits are fully 3D. `AstroNode` positions are derived using 3D Keplerian elements relative to their parent body.')

content = content.replace('- **Invariant 1 (Strict Coplanar Orbit).** All `AstroNode` orbits are strictly 2D and coplanar. The `z` or inclination coordinate for any macroscopic orbit is exactly `0.0`.', '- **Invariant 1 (3D Orbital Support).** All `AstroNode` orbits fully support 3D Keplerian mechanics. Inclination and longitude of the ascending node are fully integrated.')

content = content.replace('### 2.2 2D Coplanar Keplerian Position', '### 2.2 3D Keplerian Position')

rep_old = '''- **Parent-Relative 2D Position.**
  `X_rel = r * cos(nu + varpi)`
  `Y_rel = r * sin(nu + varpi)`
  `Z_rel = 0.0`'''

rep_new = '''- **Parent-Relative 3D Position.**
  Let `i` = inclination, `Omega` = longitude of ascending node, `omega` = argument of periapsis.
  `X_rel = r * (cos(Omega) * cos(omega + nu) - sin(Omega) * sin(omega + nu) * cos(i))`
  `Y_rel = r * (sin(Omega) * cos(omega + nu) + cos(Omega) * sin(omega + nu) * cos(i))`
  `Z_rel = r * (sin(i) * sin(omega + nu))`'''
content = content.replace(rep_old, rep_new)

content = re.sub(r'- \*\*Trap 1.*?Z_rel = 0\.0`\)\.\n', '', content, flags=re.DOTALL)

with open('docs/ssot/SSOT-PHY-004-AstroNode-Architecture.md', 'w', encoding='utf-8') as f:
    f.write(content)
