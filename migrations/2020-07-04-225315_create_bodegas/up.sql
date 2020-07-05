CREATE TABLE stores (
  id SERIAL PRIMARY KEY,
  title VARCHAR(255),
  lat DOUBLE PRECISION NOT NULL,
  lng DOUBLE PRECISION NOT NULL,
  address VARCHAR(255),
  phone VARCHAR(255),
  country_code VARCHAR(255),
  icon VARCHAR(255),
  code VARCHAR(255),
  type VARCHAR(255),
  delivery INT,
  store_url VARCHAR(255),
  promo1 VARCHAR(255),
  promo2 VARCHAR(255),
  promo3 VARCHAR(255),
  data VARCHAR(255)
);
