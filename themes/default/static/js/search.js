(function() {
    'use strict';

    var searchInput = document.getElementById('search-input');
    var searchResults = document.getElementById('search-results');
    var searchIndex = null;

    if (!searchInput || !searchResults) return;

    // Load search index
    function loadIndex() {
        if (searchIndex !== null) return;
        var xhr = new XMLHttpRequest();
        xhr.open('GET', '/search_index.json', true);
        xhr.onreadystatechange = function() {
            if (xhr.readyState === 4 && xhr.status === 200) {
                try {
                    searchIndex = JSON.parse(xhr.responseText);
                } catch (e) {
                    searchIndex = [];
                }
            }
        };
        xhr.send();
    }

    function search(query) {
        if (!searchIndex || !query) return [];
        var q = query.toLowerCase().trim();
        if (q.length < 2) return [];

        var terms = q.split(/\s+/);
        var results = [];

        for (var i = 0; i < searchIndex.length; i++) {
            var entry = searchIndex[i];
            var text = (entry.title + ' ' + entry.body + ' ' + entry.description +
                       ' ' + entry.categories.join(' ') + ' ' + entry.tags.join(' ')).toLowerCase();

            var matches = true;
            for (var j = 0; j < terms.length; j++) {
                if (text.indexOf(terms[j]) === -1) {
                    matches = false;
                    break;
                }
            }
            if (matches) {
                results.push(entry);
            }
            if (results.length >= 10) break;
        }

        return results;
    }

    function renderResults(results) {
        if (results.length === 0) {
            searchResults.innerHTML = '<div class="search-result-item"><p>No results found</p></div>';
            searchResults.classList.add('active');
            return;
        }

        var html = '';
        for (var i = 0; i < results.length; i++) {
            var r = results[i];
            var desc = r.description || r.body.substring(0, 120) + '...';
            html += '<div class="search-result-item">' +
                    '<a href="' + r.url + '">' + escapeHtml(r.title) + '</a>' +
                    '<p>' + escapeHtml(desc) + '</p></div>';
        }
        searchResults.innerHTML = html;
        searchResults.classList.add('active');
    }

    function escapeHtml(text) {
        var div = document.createElement('div');
        div.appendChild(document.createTextNode(text));
        return div.innerHTML;
    }

    searchInput.addEventListener('focus', loadIndex);

    searchInput.addEventListener('input', function() {
        var query = this.value;
        if (query.length < 2) {
            searchResults.classList.remove('active');
            return;
        }
        var results = search(query);
        renderResults(results);
    });

    document.addEventListener('click', function(e) {
        if (!searchInput.contains(e.target) && !searchResults.contains(e.target)) {
            searchResults.classList.remove('active');
        }
    });

    searchInput.addEventListener('keydown', function(e) {
        if (e.key === 'Escape') {
            searchResults.classList.remove('active');
            this.blur();
        }
    });
})();
