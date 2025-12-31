export class Time {
  constructor(
    private _hours: number,
    private _minutes: number
  ) {}

  public get minutes(): number {
    return this._minutes;
  }

  public set minutes(value: number) {
    try {
      Time.validateMinutes(value);
      this._minutes = value;
    } catch (error) {
      console.error(error);
    }
  }

  public get hours(): number {
    return this._hours;
  }

  public set hours(value: number) {
    try {
      Time.validateHours(value);
      this._hours = value;
    } catch (error) {
      console.error(error);
    }
  }

  private static validateHours(value: number) {
    if (isNaN(value)) throw new Error("Hours should be a number");

    if (value > 23 || value < 0)
      throw new Error(`Error: hours should be between 23 and 0 got ${value}`);
  }

  private static validateMinutes(value: number) {
    if (isNaN(value)) throw new Error("minutes should be a number");

    if (value > 60 || value < 0)
      throw new Error(`Error: minutes should be between 60 and 0 got ${value}`);
  }

  private static validate(hours: number, minutes: number) {
    Time.validateHours(hours);
    Time.validateMinutes(minutes);
  }

  public static fromString(value: string) {
    const splited = value.split(":");

    if (splited.length > 2 || splited.length < 2)
      throw new Error("Invalid time string");

    const hours = parseInt(splited[0]);
    const minutes = parseInt(splited[1]);

    try {
      Time.validate(hours, minutes);
      return new Time(hours, minutes);
    } catch (error) {
      console.error(error);
    }
  }

  public toString(): string {
    const formatedHours = this.hours < 10 ? `0${this.hours}` : this.hours;
    const formatedMinutes =
      this.minutes < 10 ? `0${this.minutes}` : this.minutes;
    return `${formatedHours}:${formatedMinutes}`;
  }
}
