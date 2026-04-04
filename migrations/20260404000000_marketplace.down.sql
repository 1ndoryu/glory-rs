/* [044A-38] Rollback marketplace: eliminar todas las tablas y tipos en orden inverso */

DROP TABLE IF EXISTS activity_log;
DROP TABLE IF EXISTS notifications;
DROP TABLE IF EXISTS order_delegations;
DROP TABLE IF EXISTS order_reviews;
DROP TABLE IF EXISTS order_refunds;
DROP TABLE IF EXISTS order_payments;
DROP TABLE IF EXISTS phase_deliverables;
DROP TABLE IF EXISTS order_phases;
DROP TABLE IF EXISTS orders;
DROP TABLE IF EXISTS service_plan_phases;
DROP TABLE IF EXISTS service_plans;
DROP TABLE IF EXISTS services;
DROP TABLE IF EXISTS employee_profiles;
DROP TABLE IF EXISTS user_profiles;

ALTER TABLE users DROP COLUMN IF EXISTS status;
ALTER TABLE users DROP COLUMN IF EXISTS email_verification_token;
ALTER TABLE users DROP COLUMN IF EXISTS email_verified;
ALTER TABLE users DROP COLUMN IF EXISTS active_role;
ALTER TABLE users DROP COLUMN IF EXISTS role;

DROP TYPE IF EXISTS delegation_status;
DROP TYPE IF EXISTS refund_status;
DROP TYPE IF EXISTS payment_status;
DROP TYPE IF EXISTS phase_status;
DROP TYPE IF EXISTS payment_mode;
DROP TYPE IF EXISTS order_status;
DROP TYPE IF EXISTS user_role;
