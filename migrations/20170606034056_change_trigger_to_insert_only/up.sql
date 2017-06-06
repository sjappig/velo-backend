DROP TRIGGER calculate_elo ON games;
CREATE TRIGGER calculate_elo BEFORE INSERT ON games
FOR EACH ROW EXECUTE PROCEDURE calculate_elo();
