CREATE OR REPLACE FUNCTION gen_objectid() RETURNS text
     AS 'libgen_objectid.so', 'gen_objectid'
     LANGUAGE C STRICT;