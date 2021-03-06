#!/usr/bin/env bash

set -e

postId="$1"

if [[ -z "${postId}" ]]; then
    echo "Usage:"
    echo "  ./render post-id"
    exit 0
fi

imagesPath="../../${postId}/images.pdf"
labelsPath="../../${postId}/labels.txt"
postPath="../../../static/resources/${postId}"

if [[ ! -f "${imagesPath}" ]]; then
    echo "error: Source PDF file (${imagesPath}) does not exist"
    exit 1
fi

if [[ ! -f "${labelsPath}" ]]; then
    echo "error: Source labels file (${labels_file}) does not exist"
    exit 1
fi

if [[ ! -d "${postPath}" ]]; then
    echo "error: Post directory (${postPath}) does not exist"
    exit 1
fi

echo "[+] Loading PDF"

pages=$(
    pdftk "${imagesPath}" dump_data |
        awk '/NumberOfPages/ { print $2 }'
)

if [[ -z "${pages}" ]]; then
    exit 1
fi

echo " -  Found ${pages} pages"
echo

echo "[+] Loading labels"

readarray -t labels < "${labelsPath}"

for pageId in $(seq 1 $pages); do
    pageFile="${labels[((pageId - 1))]}"
    pagePath="${postPath}/${page_file}.png"

    if [[ -z "$pageFile" ]]; then
      echo "error: Missing label for page ${pageId}"
      exit 1
    fi
done

echo
echo "[+] Converting pages"

nproc=4

for pageId in $(seq 1 $pages); do
    {
        pageFile="${labels[((pageId - 1))]}"
        pagePath="${postPath}/${pageFile}.png"

        echo " -  Converting page ${pageId} (${pageFile})"

        inkscape \
            --export-dpi=2000 \
            --export-filename="${pagePath}" \
            --pdf-page="${pageId}" \
            "${imagesPath}" &> /dev/null

        convert \
            "${pagePath}" \
            -fuzz 5% \
            -trim \
            -resize "1000>" \
            "${pagePath}" &> /dev/null
    } &

    (( ++count % nproc == 0)) && wait
done

wait

echo
echo "[+] Finished"
