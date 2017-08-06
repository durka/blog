%% load

data = [csvread('du.time') csvread('v1.time') csvread('v2.time') csvread('v3.time')];
cols = {'du', 'v1', 'v2', 'v3'};

%% plot

figure
boxplot(data, cols)
ylabel('Time (s)')
set(gcf, 'Color', [.3 .3 .3]);
set(gca, 'Color', [.3 .3 .3]);
set(gca, 'XColor', 'white');
set(gca, 'YColor', 'white');
export_fig('boxplot.png')

%% statistics

for c=1:length(cols)
    fprintf('%s: mean=%g, std=%g\n', cols{c}, mean(data(:,c)), std(data(:,c)));
end

fprintf('\nSigned-rank\n\t');
for c=1:length(cols)
    fprintf('%s\t', cols{c});
end
fprintf('\n');
for c=1:length(cols)
    fprintf('%s\t', cols{c});
    for d=1:length(cols)
        fprintf('%1.3f\t', signrank(data(:,c), data(:,d)));
    end
    fprintf('\n');
end
