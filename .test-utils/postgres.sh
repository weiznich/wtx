#!/usr/bin/env bash
set -euxo pipefail
echo "-----BEGIN CERTIFICATE-----
MIIDGzCCAgOgAwIBAgIUCGrQGG3KaDylCVcMoOT55LPuySEwDQYJKoZIhvcNAQEL
BQAwHTELMAkGA1UEBhMCRkkxDjAMBgNVBAMMBXZhaGlkMB4XDTIzMTIwODEzMjM0
M1oXDTI0MTIwNzEzMjM0M1owHTELMAkGA1UEBhMCRkkxDjAMBgNVBAMMBXZhaGlk
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAxWGYl3dgWjAzriasTKV4
BFBTt+3fPN/rk1WG+AwiS6aIBmbUow3NW3rsEDAYFp7YVA5NIENgnLPbSXQZMzu4
0sZJrQfEwuaFrE+x4kwafG3N+6OHbyZolCqMBSipF/krNWZi5t7Cxq2io37ckxL1
1gIFZa5Fyw2MUTFx4zucSbqHS6LSpBeQLN4Q1O+dXTIyjGCf6nq86ZXzhcX2hsfF
7zWysMEuh5hKYMKRWNAHbN+8TPfO0Ipv8V9tD27mU2W/Rxegn4ild470eoJPTS8G
g4Ao/mXoUFHtkG52PkCOMScIsWfgxbud9rYu0eREbIXYC5HGYGd35wYhenK7COCs
DwIDAQABo1MwUTAdBgNVHQ4EFgQU4QHDEqHtxoaOEV+ITLyOuPerF30wHwYDVR0j
BBgwFoAU4QHDEqHtxoaOEV+ITLyOuPerF30wDwYDVR0TAQH/BAUwAwEB/zANBgkq
hkiG9w0BAQsFAAOCAQEAuEJ5zEyLvjdaBOlohXJ72bJtSkeaJyznt9tzogJydM0w
eQI/fPgc4Bya5PMZt475nh1RvxMRVprMENDmCDc+LsQxkgQnntx5mO4vt02Agq96
d4qOyOddBlcXG7uRpmmz+ULXVUv+9cZSUMYPfS2NM5IdrxnRzrPoRRELXZtN5S9F
a7bzQlzR2bC8iNCKEW3gBvzPh2nq5zN3Wg9UyzSRAkrmyFK76TM1CX7gCUnVLOIk
h1hODKf9p1sPodeZiIK9+duoZLyJm3FjgrKROdMZK0ZGDLqdH3l2BMewuJ0SSZyQ
5lGNK4+L1tb9eO9VDlKi/XTU4yy5Z2TT3WigIzNGxg==
-----END CERTIFICATE-----" > $PGDATA/root-ca.crt
echo "-----BEGIN CERTIFICATE-----
MIIDMTCCAhmgAwIBAgIUcgkAxzOBcae+k90MUYESx6VwrUAwDQYJKoZIhvcNAQEL
BQAwHTELMAkGA1UEBhMCRkkxDjAMBgNVBAMMBXZhaGlkMB4XDTIzMTIwODEzMjM0
M1oXDTI0MTIwNzEzMjM0M1owHTELMAkGA1UEBhMCRkkxDjAMBgNVBAMMBXZhaGlk
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAmdl6ZBBWZu6CfbGaPdLA
5+Uq4jn5Jb2TvFltaGa+4e4HYw9camwMWYgotWqEn/ipqeZAwknkQu1Iwdd40tu3
HwP2Fe2gcnOClSMKaJVjhxA1BL3VKVzfoxGAUAzfh2IouZcZaLXS2Jb+XBOeFxX5
rY8LYlGZP7PwG/SwFsy/BMIFyyORQnTWmnKcw8KHGxG0LhjBG1T+BEsPrP6ZVCp2
hERZ3IupnyCc2n4EQ+yWXjx+GY0rVzP1prbEWcRwH2oTNGw0/+DInKTM57JaQm3B
uhPGF6gYXttP+SBUV8KwpS4fdO6vNPGu6HWme495hp2Op00P+gDGBnzYxAQi/K8j
SQIDAQABo2kwZzAfBgNVHSMEGDAWgBThAcMSoe3Gho4RX4hMvI6496sXfTAJBgNV
HRMEAjAAMBoGA1UdEQQTMBGCCWxvY2FsaG9zdIcEfwAAATAdBgNVHQ4EFgQU2WM6
4Ry0LC8QKaJLMuD9PC6T+28wDQYJKoZIhvcNAQELBQADggEBAJNjnBoWezMx6ppT
qfize2pFHABnJIK9m8ANwcYIumSMl9V0/1R1wzwsOU3qumg54RhJok0T4ugSQaBU
p3PtSEla3rRWhnbcA++29Efb3waBIm7oG5owHpT0VIfB8QevS8BmVIPib5DJLhCi
4tAIjnoxfD9IfjB59d1ykgHITTF3fC2jhJVnwAc8ppa7wBVQDavsYj0rLgLchLo8
RMv4FCqQKc3vaG84ksouRslhZ8brifgOvyRKwfNMuz5cY7E4MK//QCwzlBuwzqcN
JmG0EXgHbwE0uUJBnxlyjR41/y2DkTAwqut+vwWSPXubvwVAYPfw6cYd67ZRBJ/A
bX4bifw=
-----END CERTIFICATE-----" > $PGDATA/cert.pem
echo "-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCZ2XpkEFZm7oJ9
sZo90sDn5SriOfklvZO8WW1oZr7h7gdjD1xqbAxZiCi1aoSf+Kmp5kDCSeRC7UjB
13jS27cfA/YV7aByc4KVIwpolWOHEDUEvdUpXN+jEYBQDN+HYii5lxlotdLYlv5c
E54XFfmtjwtiUZk/s/Ab9LAWzL8EwgXLI5FCdNaacpzDwocbEbQuGMEbVP4ESw+s
/plUKnaERFnci6mfIJzafgRD7JZePH4ZjStXM/WmtsRZxHAfahM0bDT/4MicpMzn
slpCbcG6E8YXqBhe20/5IFRXwrClLh907q808a7odaZ7j3mGnY6nTQ/6AMYGfNjE
BCL8ryNJAgMBAAECggEACvOBE8hZ7h/CKCJRQV9UHe2hHNNVmcshzgFrmUffBIQG
AJYiVOz2ABWCGE/Juv1fTlm2eF38IotzZ2DoF9JN+aY1iSAOELeKLwV8gy9HE2Ei
9QByQOx8esYPJSOVcbSaA4Pu6hYvOP2eg31d/nSw/hq1i70VNIShbcwAEM9oNXVh
5TzGM838Bit/DjWLBbxXO/FdFIsTQf68AR9I5PbbET3RNPmET6sBiOx9eQ5Cv+A0
JiPMiWuZQb2N6RepJoLC5qlZeziAoF6ThuhSN+ptyYhe+l7h0WEGkUZLUXpVBPiy
Z3PaBAbAQ1+GTtmAyB8CzndoTWZJYWn2kjFZPgtpAQKBgQDFbHnAP6R84ysjQrw2
oAiEOfaPITM+uzP+DVm4S70tJHWfhJfXfd+77WBUOmSr3VZm9c7/ycG6wCJXzgFw
htBaBMeEAZw90E/0NpF1Y2ZCObipUYzgaEOE4d1/0uCusp2AM/NK111cKKR8QaGK
GsXpQZ6G2cTiseY/L4S0CQsVoQKBgQDHf0c70wGHh5AQbmN7fRntJzDmouyVIuV1
1h7sixoliM6J5hktGvrmTtmeZE/JrgKR71airYqq/spxHr7CdDjX5NUjw5r7P2WW
7EtAgJsT8fEsGUCUTnWNnMikWYyloNBsUezE5oJ478p8y5UqNwHnt4vPtT8xU6Ly
sPu0kZhcqQKBgBKg8bwaRUMjjGZ/XZMp9qPDdGh9EYDR2U4XpGenfepMjmzG7iz/
S6VH+Nb5y5YMBKMifq3UM9uTVapDXg9oKTDVUp46KKwMg/7fR0p0XqnRGIOhllF2
hSh8x5HQLDNP11zJiiN9vul7TEoNR1jovdQMxRCka9eYQcmkijkSwWEBAoGAd8em
yHefonU1fXgaCQoK6AYw6TmHXN/v77lZJeM7FrA2ejuKgDeDz4YmSWCnO0tcRazz
UodqC9MhT5wLIwvPGWlOw8NtKU9eOqizYg4Vjnskt7qNuL4G/LjXCz6tIMEWgwMJ
awuP6Pbol9dQP2F5plvg+Rw0zfQOkTLMXhVQ6skCgYEAsygCHEbL3ggvW5Zb30wr
kuLgLTG1fXb2jU28FspAD7IKyu4r+4gjfMnroDj50W6KbHtfSBJvHYU5EPNFVRBp
WDVgGLd7qcUT9XE5uTjSI3C5+NAbXpX04UP5q4NDmEJC9UgMFsIo+qdO+zlnaeRj
2Y6qR8subc7S2HCR6O07l+Q=
-----END PRIVATE KEY-----" > $PGDATA/key.pem
chmod 0600 $PGDATA/key.pem
cat >> "$PGDATA/postgresql.conf" <<-EOF
ssl = on
ssl_ca_file = 'root-ca.crt'
ssl_cert_file = 'cert.pem'
ssl_key_file = 'key.pem'
EOF
cat > "$PGDATA/pg_hba.conf" <<-EOF
# TYPE  DATABASE        USER            ADDRESS                 METHOD
host    all             wtx_md5        0.0.0.0/0            md5
host    all             wtx_scram      0.0.0.0/0            scram-sha-256
host    all             wtx_md5        ::0/0                md5
host    all             wtx_scram      ::0/0                scram-sha-256

local    all             all                                md5
EOF

psql -v ON_ERROR_STOP=1 --username $POSTGRES_USER <<-EOF
    CREATE ROLE wtx_md5 PASSWORD 'wtx' LOGIN;
    ALTER DATABASE wtx OWNER TO wtx_md5;

    SET password_encryption TO 'scram-sha-256';
    CREATE ROLE wtx_scram PASSWORD 'wtx' LOGIN;
EOF