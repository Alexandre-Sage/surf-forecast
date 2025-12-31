export class Coordinates {
  constructor(
    private readonly _latitude: number,
    private readonly _longitude: number
  ) {}

  public latitude(): number {
    return this._latitude;
  }

  public longitude(): number {
    return this._longitude;
  }
  static fromString(latitude: string, longitude: string) {
    const lat = parseFloat(latitude);
    const lng = parseFloat(longitude);
    //TODO add notif insead of this or error boundaries
    if (Number.isNaN(lat)) throw new Error("Latitude not a number");

    if (Number.isNaN(lng)) throw new Error("Longitude not a number");

    return new Coordinates(lat, lng);
  }
}
