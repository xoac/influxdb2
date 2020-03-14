set -e
set -x

INFLUX_ORG=primary_org
INFLUX_BUCKET=primary_bucket
INFLUX_USER=user

CONTAINER=$(docker run --rm -d -p 9999:9999 quay.io/influxdb/influxdb:2.0.0-beta)
f() {
  echo "Stopping created container.."
  docker stop $CONTAINER
}
trap f ERR

sleep 7
docker exec $CONTAINER influx setup -f -b $INFLUX_BUCKET -u $INFLUX_USER -p password -o $INFLUX_ORG
sleep 1
INFLUX_TOKEN=$(docker exec $CONTAINER influx auth list | awk 'NR==2{print $2}')
INFLUX_ORG=$INFLUX_ORG INFLUX_TOKEN=$INFLUX_TOKEN INFLUX_BUCKET=$INFLUX_BUCKET cargo test --verbose --all
docker stop $CONTAINER
