# Wave Height Calculation Formulas

This document describes various mathematical formulas for calculating combined wave/swell height based on primary swell, secondary swell, and wind wave characteristics.

## Variables

- $H_p$ = Primary swell height (meters)
- $H_s$ = Secondary swell height (meters)
- $H_w$ = Wind wave height (meters)
- $T_p$ = Primary swell period (seconds)
- $T_s$ = Secondary swell period (seconds)
- $T_w$ = Wind wave period (seconds)
- $\theta_p$ = Primary swell direction (degrees, 0-360°)
- $\theta_s$ = Secondary swell direction (degrees, 0-360°)
- $\theta_w$ = Wind wave direction (degrees, 0-360°)
- $H_{total}$ = Combined/Total wave height (meters)

## Formula 1: Root Sum of Squares (RSS)

The simplest and most commonly used formula. Assumes waves are independent and accounts for phase differences.

$$
H_{total} = \sqrt{H_p^2 + H_s^2 + H_w^2}
$$

### When to use:
- Quick calculations
- When directional information is not critical
- Standard practice in many forecasting systems

---

## Formula 2: Directional Root Sum of Squares

Accounts for wave direction to determine how waves interact. Waves from similar directions add constructively, while waves from opposite directions can cancel out.

### Step 1: Calculate angle differences

$$
\Delta\theta_{ps} = \min(|\theta_p - \theta_s|, 360° - |\theta_p - \theta_s|)
$$

$$
\Delta\theta_{pw} = \min(|\theta_p - \theta_w|, 360° - |\theta_p - \theta_w|)
$$

$$
\Delta\theta_{sw} = \min(|\theta_s - \theta_w|, 360° - |\theta_s - \theta_w|)
$$

### Step 2: Calculate directional weights

$$
w_{ps} = \cos\left(\frac{\Delta\theta_{ps} \cdot \pi}{180°}\right)
$$

$$
w_{pw} = \cos\left(\frac{\Delta\theta_{pw} \cdot \pi}{180°}\right)
$$

$$
w_{sw} = \cos\left(\frac{\Delta\theta_{sw} \cdot \pi}{180°}\right)
$$

### Step 3: Calculate combined height

$$
H_{total} = \sqrt{H_p^2 + H_s^2(1 + w_{ps}) + H_w^2(1 + w_{pw})}
$$

### Interpretation:
- $w = 1$: Waves from same direction (0° difference) → fully additive
- $w = 0$: Waves perpendicular (90° difference) → independent
- $w = -1$: Waves from opposite directions (180° difference) → subtractive

---

## Formula 3: Wave Energy-Based Calculation

Uses wave energy (proportional to $H^2 \cdot T$) to combine waves, then converts back to equivalent height.

### Step 1: Calculate wave energy for each source

$$
E_p = H_p^2 \cdot T_p
$$

$$
E_s = H_s^2 \cdot T_s
$$

$$
E_w = H_w^2 \cdot T_w
$$

### Step 2: Calculate total energy

$$
E_{total} = E_p + E_s + E_w
$$

### Step 3: Calculate average period

$$
\bar{T} = \frac{T_p + T_s + T_w}{3}
$$

### Step 4: Convert energy back to equivalent height

$$
H_{total} = \sqrt{\frac{E_{total}}{\bar{T}}}
$$

---

## Formula 4: Significant Wave Height (Hs)

Represents the average height of the highest one-third of waves. For combined sources:

$$
H_s = \sqrt{H_p^2 + H_s^2 + H_w^2}
$$

This is equivalent to Formula 1 (RSS) when combining independent wave sources.

---

## Formula 5: Weighted Combination by Period

Gives more weight to longer period swells, as they typically have more energy:

### Step 1: Calculate period weights

$$
w_p = \frac{T_p}{T_p + T_s + T_w}
$$

$$
w_s = \frac{T_s}{T_p + T_s + T_w}
$$

$$
w_w = \frac{T_w}{T_p + T_s + T_w}
$$

### Step 2: Calculate weighted combined height

$$
H_{total} = \sqrt{(w_p H_p)^2 + (w_s H_s)^2 + (w_w H_w)^2}
$$

---

## Formula 6: Maximum Envelope Method

Takes the maximum of all sources plus a fraction of others:

$$
H_{total} = \max(H_p, H_s, H_w) + 0.5 \cdot \sqrt{H_{other1}^2 + H_{other2}^2}
$$

Where $H_{other1}$ and $H_{other2}$ are the two smaller wave heights.

---

## Recommended Formula

For most practical applications, **Formula 1 (RSS)** is recommended:

$$
H_{total} = \sqrt{H_p^2 + H_s^2 + H_w^2}
$$

### Advantages:
- Simple and computationally efficient
- Widely used in forecasting systems
- Accounts for phase differences between waves
- Conservative estimate (doesn't overestimate)

### Limitations:
- Doesn't account for wave direction
- Assumes waves are independent
- May underestimate when waves align perfectly

---

## Example Calculation

Given:
- Primary swell: $H_p = 2.5$ m, $T_p = 12$ s, $\theta_p = 180°$
- Secondary swell: $H_s = 1.5$ m, $T_s = 8$ s, $\theta_s = 190°$
- Wind wave: $H_w = 0.8$ m, $T_w = 5$ s, $\theta_w = 200°$

### Using Formula 1 (RSS):

$$
H_{total} = \sqrt{2.5^2 + 1.5^2 + 0.8^2} = \sqrt{6.25 + 2.25 + 0.64} = \sqrt{9.14} \approx 3.02 \text{ m}
$$

### Using Formula 2 (Directional RSS):

$$
\Delta\theta_{ps} = |180° - 190°| = 10°
$$

$$
w_{ps} = \cos\left(\frac{10° \cdot \pi}{180°}\right) = \cos(0.175) \approx 0.985
$$

$$
H_{total} = \sqrt{2.5^2 + 1.5^2(1 + 0.985) + 0.8^2} \approx 3.15 \text{ m}
$$

---

## References

- [NOAA Wave Height Calculations](https://www.ndbc.noaa.gov/wavecalc.shtml)
- [WMO Guide to Wave Analysis and Forecasting](https://library.wmo.int/)
- Standard practices in oceanographic forecasting systems
