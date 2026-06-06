function updateArtifactIds() {
    var checkboxes = document.querySelectorAll('.checkbox-list input[type="checkbox"]');
    var ids = [];
    checkboxes.forEach(function(cb) {
        if (cb.checked) {
            ids.push(cb.value);
        }
    });
    document.getElementById('artifact_ids').value = ids.join(',');
}

document.addEventListener('DOMContentLoaded', function() {
    var dateInputs = document.querySelectorAll('input[type="date"]');
    dateInputs.forEach(function(input) {
        if (!input.value) {
            var today = new Date().toISOString().split('T')[0];
            input.value = today;
        }
    });
});
