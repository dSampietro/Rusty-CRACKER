filename="syn_1M_10M"

for i in 8; do
    flamegraph\
    -o ../profiling/libV2_"$filename"_"$i"threads.svg\
    -- ./release/rayon_main \
        -f ../files/syn/$filename.mtx\
        --num_thread $i
done