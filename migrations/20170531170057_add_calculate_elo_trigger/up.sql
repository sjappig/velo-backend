CREATE OR REPLACE FUNCTION calculate_elo() RETURNS TRIGGER
AS $calculate_elo$
DECLARE
    oldelo1 double precision;
    oldelo2 double precision;
    r1 double precision;
    r2 double precision;
    e1 double precision;
    e2 double precision;
    elo1 double precision;
    elo2 double precision;
    k double precision;
BEGIN
    k := 32.0;
    SELECT elo FROM players WHERE id = NEW.winner INTO oldelo1;
    SELECT elo FROM players WHERE id = NEW.loser INTO oldelo2;
    SELECT pow(10, (oldelo1 / 400.0)) INTO r1;
    SELECT pow(10, (oldelo2 / 400.0)) INTO r2;
    SELECT r1 / (r1 + r2) INTO e1;
    SELECT r2 / (r1 + r2) INTO e2;
    SELECT oldelo1 + (k * (1.0 - e1)) INTO elo1;
    SELECT oldelo2 + (k * (0.0 - e2)) INTO elo2;

    UPDATE players SET elo = elo1 WHERE id = NEW.winner;
    UPDATE players SET elo = elo2 WHERE id = NEW.loser;

    RETURN NEW;
END;
$calculate_elo$ LANGUAGE plpgsql;

CREATE TRIGGER calculate_elo
BEFORE INSERT OR UPDATE ON games
FOR EACH ROW EXECUTE PROCEDURE calculate_elo();
